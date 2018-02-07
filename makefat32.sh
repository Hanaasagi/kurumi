#!/bin/bash

set -e

IMG="disk.img" # image name
SIZE=64        # image size
SOURCE="test"  # source dir

if [ -e "$IMG" ]; then
  echo "ERROR: $IMG already exists."
  exit 1
fi

SECTOR_SIZE=512
LOGICAL_SECTOR_SIZE=512
NEW_DISKLABEL=o
NEW_PARTITION=n
PRIMARY=p
FIRST=1
OFFSET_8MB=16384
SET_PARTITION_TYPE=t
WIN95_FAT32=b
WRITE=w

relpath() {
  full=$1
  if [ "$full" == "$SOURCE" ]; then
    echo ""
  else
    base=${SOURCE%%/}/
    echo "${full##$base}"
  fi
}

DISK_SIZE=$(echo $(expr 8 + $SIZE))
PARTITION=${IMG}.partition

# make image file
fallocate -l ${DISK_SIZE}M "$IMG"
echo "$NEW_DISKLABEL
$NEW_PARTITION
$PRIMARY
$FIRST
$OFFSET_8MB

$SET_PARTITION_TYPE
$WIN95_FAT32
$WRITE
" | /sbin/fdisk "$IMG" >/dev/null

# make partition
fallocate -l ${SIZE}M "$PARTITION"
/sbin/mkfs.fat -F32 -S"$LOGICAL_SECTOR_SIZE" "$PARTITION" >/dev/null

# copy file
find "$SOURCE" -type d | while read dir; do
  target=$(relpath $dir)
  [ -z "$target" ] && continue
  mmd -i "$PARTITION" "::$target"
done
find $SOURCE -type f | while read file; do
  target=$(relpath $file)
  mcopy -i "$PARTITION" "$file" "::$target"
done

# insert partition
dd if="$PARTITION" of="$IMG" bs=$SECTOR_SIZE seek=$OFFSET_8MB >/dev/null 2>&1

# clean
rm -f "$PARTITION"
