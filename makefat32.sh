#!/bin/bash

set -e

IMG="disk.img" # image name
SIZE=64        # image size
SOURCE="test"  # source dir

LOGICAL_SECTOR_SIZE=512
OFFSET=0

if [ -e "$IMG" ]; then
  echo "ERROR: $IMG already exists."
  exit 1
fi

relpath() {
  full=$1
  if [ "$full" == "$SOURCE" ]; then
    echo ""
  else
    base=${SOURCE%%/}/
    echo "${full##$base}"
  fi
}

# generate source dir
(mkdir $SOURCE && cd "$_" && \
    echo "Hello World" >> README && \
    mkdir program && cd "$_" && \
    echo "I love Rust" >> rust.txt)

# make img
fallocate -l ${SIZE}M "$IMG"
/sbin/mkfs.fat -F32 -S"$LOGICAL_SECTOR_SIZE" "$IMG" >/dev/null

# copy file
find "$SOURCE" -type d | while read dir; do
  target=$(relpath $dir)
  [ -z "$target" ] && continue
  mmd -i "$IMG" "::$target"
done
find $SOURCE -type f | while read file; do
  target=$(relpath $file)
  mcopy -i "$IMG" "$file" "::$target"
done

# clean
rm -rf $SOURCE
