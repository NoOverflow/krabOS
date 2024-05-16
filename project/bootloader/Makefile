all: bootloader

bootloader: bootloader.asm
	nasm -f bin bootloader.asm -o bootloader.bin

debug: bootloader
	qemu-system-i386 -drive file=bootloader.bin,format=raw
