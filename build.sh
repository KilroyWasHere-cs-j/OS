#!/bin/bash

echo "Building OS"
echo "FYI this bitch is being built in 32 bit"

nasm -f elf32 loader.s

ld -T link.ld -melf_i386 loader.o -o kernel.elf

mkdir -p iso/boot/grub
cp stage2_eltorito iso/boot/grub/
cp kernel.elf iso/boot/
cp menu.lst iso/boot/grub/

echo "Set up done 

    genisoimage -R                              \
                -b boot/grub/stage2_eltorito    \
                -no-emul-boot                   \
                -boot-load-size 4               \
                -A os                           \
                -input-charset utf8             \
                -quiet                          \
                -boot-info-table                \
                -o os.iso                       \
                iso

echo "We done"
