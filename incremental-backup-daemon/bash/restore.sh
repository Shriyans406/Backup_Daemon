#!/bin/bash

TIMESTAMP=$1
RESTORE_TARGET=$2

INDEX_FILE="storage/index.json"

if [ -z "$TIMESTAMP" ] || [ -z "$RESTORE_TARGET" ]; then
    echo "Usage: restore.sh <timestamp> <target_directory>"
    exit 1
fi

echo "[RESTORE] Starting restore for timestamp: $TIMESTAMP"

# Find backup directory from index
BACKUP_DIR=$(grep -A2 "$TIMESTAMP" "$INDEX_FILE" | grep backup_dir | cut -d '"' -f4)

if [ -z "$BACKUP_DIR" ]; then
    echo "[RESTORE] ERROR: Backup not found"
    exit 1
fi

FULL_BACKUP_PATH="storage/$BACKUP_DIR"

echo "[RESTORE] Backup source: $FULL_BACKUP_PATH"
echo "[RESTORE] Restoring into: $RESTORE_TARGET"

mkdir -p "$RESTORE_TARGET"

cp -r "$FULL_BACKUP_PATH"/* "$RESTORE_TARGET" 2>/dev/null

echo "[RESTORE] Restore completed"