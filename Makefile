build:
	zig build

build-image:
	mkdir -p iso/boot
	mkdir -p iso/boot/grub/
	cp zig-out/bin/kernel.elf iso/boot/kernel.elf
	cp grub.cfg iso/boot/grub/grub.cfg
	grub-mkrescue -o kernel.iso iso

run:
	qemu-system-i386 -cdrom ./kernel.iso

clean:
	rm -rf ./.zig-cache
	rm -rf ./zig-out
	rm -rf ./iso
	rm -f kernel.iso
	
