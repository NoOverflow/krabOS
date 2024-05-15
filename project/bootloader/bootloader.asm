;
; KrabbOS boot loader
;
; This file is part of KrabbOS.
;
; Use real mode to load the kernel from disk, switch to protected mode and jump to the kernel entry point.
;

[ORG 0x7c00]
[BITS 16]

; Constants
    ; BIOS Video modes
    VIDEO_MODE_TEXT_80X25 equ 0x3

    ; Boot loader code
    KERNEL_LOAD_SEGMENT equ 0x0
    KERNEL_LOAD_OFFSET equ 0x1000

xor ax, ax ; Clear ax register
mov ds, ax ; Set data segment to 0
cld        ; Set direction flag to forward

jmp main

; Utils imports
%include "utils/real/print_string.asm"
%include "utils/real/print_hex.asm"
%include "utils/real/disk_read.asm"

error_sink:
    jmp $

disk_read_error:
    mov si, LOADING_KERNEL_DISK_FAIL_STR
    call print_string
    mov dx, ax
    call print_hex_nl
    call error_sink

; Boot loader entry point
main:
    ; Set stack pointer
    mov bp, 0x8000
    mov sp, bp

    ; Set video mode
    mov ah, 0x0
    mov al, VIDEO_MODE_TEXT_80X25
    int 0x10

    mov [BOOT_DRIVE_INDEX], dl ; Save boot drive index

    mov si, HEADER_MESSAGE_STR
    call print_string_nl

    ; Load kernel from disk
    mov si, LOADING_KERNEL_STR
    call print_string_nl

    mov bx, KERNEL_LOAD_SEGMENT
    mov es, bx
    mov bx, KERNEL_LOAD_OFFSET
    mov al, 0x20 ; Read 32 sectors (< 0x10000 limit)
    mov ch, 0x0  ; Cylinder 0
    mov cl, 0x2  ; Sector 2 (1 is the boot sector)
    mov dh, 0x0  ; Head 0
    mov dl, [BOOT_DRIVE_INDEX]
    call read_disk
    test al, al
    jnz disk_read_error

    mov dx, [KERNEL_LOAD_OFFSET]
    call print_hex_nl
    mov dx, [KERNEL_LOAD_OFFSET + 512]
    call print_hex_nl

    jmp $

; Global variables
BOOT_DRIVE_INDEX db 0

; Strings
HEADER_MESSAGE_STR db "> KrabbOS Bootloader (16 Bits mode)", 0
LOADING_KERNEL_STR db "> Loading kernel...", 0
LOADING_KERNEL_DISK_FAIL_STR db "> Error loading kernel from disk, error code: ", 0

; Boot sector signature
times 510 - ($-$$) db 0
dw 0xAA55

times 256 dw 0xdada
times 256 dw 0xface
