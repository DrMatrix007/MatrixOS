#!/bin/bash

mkdir -p iso/boot/grub

cp bin/matrix_os iso/boot/matrix_os
cp grub.cfg iso/boot/grub/grub.cfg
grub-mkrescue -o matrix_os.iso iso/