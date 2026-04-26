#!/bin/bash

TIMESTAMP=$1
RESTORE_TARGET=$2

if [ -z "$TIMESTAMP" ] || [ -z "$RESTORE_TARGET" ]; then
    echo "Usage: restore.sh <timestamp> <target_directory>"
    exit 1
fi

echo "[RESTORE] Starting restore using objects..."

SNAPSHOT_FILE="storage/snapshots/$TIMESTAMP.json"

if [ ! -f "$SNAPSHOT_FILE" ]; then
    echo "[RESTORE] ERROR: Snapshot not found"
    exit 1
fi

mkdir -p "$RESTORE_TARGET"

echo "[RESTORE] Reading snapshot: $SNAPSHOT_FILE"

# Read JSON and extract file path + hash
jq -r '.files | to_entries[] | "\(.key)|\(.value.hash)"' "$SNAPSHOT_FILE" | \
while IFS='|' read -r path hash; do

    src="storage/objects/$hash"
    dest="$RESTORE_TARGET$path"

    # Create folder structure
    mkdir -p "$(dirname "$dest")"

    # Copy file
    cp "$src" "$dest" 2>/dev/null

done

echo "[RESTORE] Restore completed successfully"