#!/bin/bash

TIMESTAMP=$1
RESTORE_TARGET=$2

INDEX_FILE="storage/index.json"

if [ -z "$TIMESTAMP" ] || [ -z "$RESTORE_TARGET" ]; then
    echo "Usage: restore.sh <timestamp> <target_directory>"
    exit 1
fi

echo "[RESTORE] Starting FULL reconstruction restore..."

mkdir -p "$RESTORE_TARGET"

# Extract all timestamps in order
ALL_TIMESTAMPS=$(grep '"timestamp"' "$INDEX_FILE" | cut -d '"' -f4)

VALID_TIMESTAMPS=()

# Collect timestamps up to target
for ts in $ALL_TIMESTAMPS; do
    VALID_TIMESTAMPS+=("$ts")

    if [ "$ts" == "$TIMESTAMP" ]; then
        break
    fi
done

# Check if timestamp found
if [[ ! " ${VALID_TIMESTAMPS[@]} " =~ " $TIMESTAMP " ]]; then
    echo "[RESTORE] ERROR: Timestamp not found in index"
    exit 1
fi

echo "[RESTORE] Applying backups in order..."

count=0

for ts in "${VALID_TIMESTAMPS[@]}"; do
    BACKUP_PATH="storage/backups/$ts"

    if [ -d "$BACKUP_PATH" ]; then
        echo "[RESTORE] Applying backup: $ts"

        cp -r "$BACKUP_PATH"/* "$RESTORE_TARGET" 2>/dev/null

        ((count++))
    fi
done

echo "[RESTORE] Reconstruction complete using $count backup layers"