;
; KrabbOS boot loader
;
; This file is part of KrabbOS.
;
; Use real mode to load the kernel from disk, switch to protected mode and jump to the kernel entry point.
;
[org 0x7C00]
[bits 16]
jmp main

; Utils imports
%include "utils/real/gdt_struct.asm"
%include "utils/real/print_string.asm"
%include "utils/real/print_hex.asm"
%include "utils/real/disk_read.asm"
%include "utils/protected/print_string.asm"

; Boot loader entry point
[bits 16]
disk_read_error:
    mov si, LOADING_KERNEL_DISK_FAIL_STR
    call print_string
    mov dx, ax
    call print_hex_nl
    jmp $

main:
    xor ax, ax ; Clear ax register
    mov ds, ax ; Set data segment to 0
    cld        ; Set direction flag to forward

    ; Setup stack
    mov bp, 0x9000
    mov sp, bp

    mov dl, [BOOT_DRIVE_INDEX]  ; Store boot drive index

    ; Clear screen
    mov ah, 0x00
    mov al, BIOS_TEXT_MODE
    int 0x10

    ; Load kernel
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

    ; Switch to 32 bits protected mode
    cli ; Disable interrupts
    lgdt [gdt_descriptor] ; Load GDT
    mov eax, cr0
    or eax, 0x1 ; Set protected mode bit
    mov cr0, eax
    jmp CODE_SEG:start_protected_mode


[bits 32]
start_protected_mode:
    ; Set up data segments
    mov ax, DATA_SEG
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; (Re)Set up stack
    mov ebp, 0x90000
    mov esp, ebp

    call main_protected_entry_point

main_protected_entry_point:
    mov ebx, MSG_PROTECTED_MODE_STR
    call print_string_pm

    call KERNEL_LOAD_OFFSET
    jmp $

; Global variables
BOOT_DRIVE_INDEX db 0

; Constants
MSG_PROTECTED_MODE_STR db "Loaded in 32 bits protected mode", 0
LOADING_KERNEL_DISK_FAIL_STR db "Error loading kernel from disk, error code: ", 0
BIOS_TEXT_MODE equ 0x3

KERNEL_LOAD_SEGMENT equ 0x0
KERNEL_LOAD_OFFSET equ 0x1000

; Boot sector signature
times 510 - ($-$$) db 0
dw 0xAA55
