all: build

kernel:
	cd kernel && cargo build --release

kernel_hook: kernel
	cd bootloader && nasm -f elf kernel_hook.asm -o kernel_hook.o

bootloader: bootloader/bootloader.asm
	cd bootloader && nasm -f bin bootloader.asm -o bootloader.bin

build: bootloader kernel
	ld -m elf_i386 -o kernel.bin -Ttext 0x1000 bootloader/kernel_hook.o kernel/target/i686-krabos/release/krabos_kernel --oformat binary
	cat bootloader/bootloader.bin kernel.bin > os.bin

debug: build
	qemu-system-i386.exe -fda os.bin
