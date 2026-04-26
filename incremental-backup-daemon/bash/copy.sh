#!/bin/bash

INPUT_FILE="../storage/temp/changed_files.txt"
OBJECTS_DIR="../storage/objects"

mkdir -p "$OBJECTS_DIR"

echo "[COPY] Using deduplicated storage..."

count=0
saved=0

while IFS= read -r file; do

    [ -z "$file" ] && continue

    # Generate hash again (same as scan)
    hash=$(sha256sum "$file" | awk '{print $1}')

    dest="$OBJECTS_DIR/$hash"

    # If file already exists, skip copy
    if [ -f "$dest" ]; then
        ((count++))
        continue
    fi

    cp "$file" "$dest" 2>/dev/null

    ((count++))
    ((saved++))

    if ((count % 50 == 0)); then
        echo "[COPY] Processed $count files..."
    fi

done < "$INPUT_FILE"

echo "[COPY] Done. Stored: $saved new files, skipped duplicates."