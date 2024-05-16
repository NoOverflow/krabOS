;
; KrabbOS boot loader
;
; This file is part of KrabbOS.
;
; Use real mode to load the kernel from disk, switch to protected mode and jump to the kernel entry point.
;

[ORG 0x7c00]
[bits 16]
jmp main

; Utils imports
%include "utils/real/gdt_struct.asm"
%include "utils/protected/print_string.asm"

; Boot loader entry point
[bits 16]
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
    mov ebx, MSG_PROTECTED_MODE
    call print_string_pm

    jmp $

; Global variables
BOOT_DRIVE_INDEX db 0

; Constants
MSG_PROTECTED_MODE db "Loaded in 32 bits protected mode", 0
BIOS_TEXT_MODE equ 0x3

KERNEL_LOAD_SEGMENT equ 0x0
KERNEL_LOAD_OFFSET equ 0x1000

; Boot sector signature
times 510 - ($-$$) db 0
dw 0xAA55
