#!/bin/bash
echo "Starting QEMU"
cd $HOME/optee/build || exit

# patch Makefile to launch qemu immediately
sed -i 's/-s -S -machine virt,secure=on -cpu cortex-a15 \\/-s -machine virt,secure=on -cpu cortex-a15 \\/' Makefile

make run-only QEMU_VIRTFS_ENABLE=y QEMU_VIRTFS_HOST_DIR=$HOME/project
