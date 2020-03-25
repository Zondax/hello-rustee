#!/bin/sh
# ATTENTION: THIS SCRIPT IS SUPPOSED TO RUN INSIDE QEMU!!

echo "Copying TAs..."
ls /mnt/ta/*.ta
cp /mnt/ta/*.ta /lib/optee_armtz/

echo
echo ------------------------ LAUNCHING HOST -------------------------------
echo

FILE=/mnt/host/hello_rustee
if test -f "$FILE"; then
  /mnt/host/hello_rustee
else
  echo "File binary does not exist. Did you compile your app?"
fi
