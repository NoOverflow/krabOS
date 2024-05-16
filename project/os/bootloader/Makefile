all: bootloader

bootloader: bootloader.asm
	nasm -f bin bootloader.asm -o bootloader.bin

debug: bootloader
	qemu-system-x86_64 -drive file=bootloader.bin,format=raw
