#!/bin/bash

count=0

TARGET_DIR=$1
OUTPUT_FILE="../storage/temp/current_state.txt"
IGNORE_FILE="../.backupignore"

> "$OUTPUT_FILE"

echo "[SCAN] Starting scan in: $TARGET_DIR"

# Build exclude arguments
EXCLUDES=()

if [ -f "$IGNORE_FILE" ]; then
    while read -r pattern; do
        EXCLUDES+=(! -path "*/$pattern/*")
    done < "$IGNORE_FILE"
fi

# Scan files safely
find "$TARGET_DIR" -type f "${EXCLUDES[@]}" -print0 2>/dev/null | \
while IFS= read -r -d '' file; do

    size=$(stat -c%s "$file" 2>/dev/null)
    mtime=$(stat -c%Y "$file" 2>/dev/null)
    hash=$(sha256sum "$file" 2>/dev/null | awk '{print $1}')


    ((count++))
if ((count % 500 == 0)); then
    echo "[SCAN] Processed $count files..."
fi

    # Skip if any value missing
    if [ -z "$size" ] || [ -z "$mtime" ] || [ -z "$hash" ]; then
        continue
    fi

    echo "$file|$size|$mtime|$hash" >> "$OUTPUT_FILE"

done

echo "[SCAN] Completed"

sort "$OUTPUT_FILE" -o "$OUTPUT_FILE"