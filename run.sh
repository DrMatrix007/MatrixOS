#!/bin/bash

./build.sh
./build_image.sh

qemu-system-i386 -cdrom ./matrix_os.iso

