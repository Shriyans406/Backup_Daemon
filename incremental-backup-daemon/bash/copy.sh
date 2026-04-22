#!/bin/bash

INPUT_FILE="../storage/temp/changed_files.txt"
BACKUP_ROOT="../storage/backups"

# Generate timestamp
TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
DEST="$BACKUP_ROOT/$TIMESTAMP"

mkdir -p "$DEST"

echo "[COPY] Starting backup to $DEST"

count=0

while IFS= read -r file; do

    # Skip empty lines
    [ -z "$file" ] && continue

    # Create destination path
    dest_path="$DEST$file"

    # Create directory structure
    mkdir -p "$(dirname "$dest_path")"

    # Copy file
    cp "$file" "$dest_path" 2>/dev/null

    ((count++))

    if ((count % 50 == 0)); then
        echo "[COPY] Copied $count files..."
    fi

done < "$INPUT_FILE"

echo "[COPY] Backup completed. Total files: $count"