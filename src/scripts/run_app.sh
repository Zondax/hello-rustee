#!/bin/sh
# ATTENTION: THIS SCRIPT IS SUPPOSED TO RUN INSIDE QEMU!!

echo "Copying TAs..."
ls /mnt/ta/*.ta
cp /mnt/ta/*.ta /lib/optee_armtz/

echo
echo ------------------------ LAUNCHING HOST -------------------------------
echo

/mnt/host/hello_rustee
