#!/bin/bash
echo "Starting QEMU"

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

cd $HOME/optee/build || exit

cp $DIR/fstab $HOME/optee/build/br-ext/board/qemu/overlay/etc/fstab
cp $DIR/inittab $HOME/optee/build/br-ext/board/qemu/overlay/etc/inittab

# patch QEMU launcher Makefiles to start emulation immediately
echo "Adding automount + autologin"
sed -i 's/-s -S -machine virt/-s -machine virt/g' qemu_v8.mk
sed -i 's/-s -S -machine virt/-s -machine virt/g' qemu.mk

make run QEMU_VIRTFS_ENABLE=y QEMU_USERNET_ENABLE=y QEMU_VIRTFS_HOST_DIR=$HOME/project
