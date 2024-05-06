;
; KrabbOS boot loader
;
; This file is part of KrabbOS.

[ORG 0x7c00]
bits 16

xor ax, ax ; Clear ax register
mov ds, ax ; Set data segment to 0
cld        ; Set direction flag to forward

jmp main

; Print a string to the screen in TTY mode
; Input:
;   si = pointer to string
print_string:
    mov ah, 0x0E ; Enable BIOS TTY
    mov cx, 0    ; Start at 0
    .loop:
        lodsb ; Load next byte from string
        test al, al
        jz .done
        int 0x10 ; print character
        jmp .loop

    .done:
        xor ax, ax
        ret

; Boot loader entry point
main:
    mov ah, 000 ; Set video mode
    mov al, 0x3 ; 80x25 text mode
    int 0x10

    mov si, boot_message
    call print_string

    jmp $

boot_message db "> KrabbOS Bootloader v0.0.1", 0

; Boot sector signature
times 510 - ($-$$) db 0
dw 0xAA55
