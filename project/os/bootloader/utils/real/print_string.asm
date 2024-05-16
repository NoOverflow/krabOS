;
; KrabbOS utils -- disk_read
;
; This file is part of KrabbOS.
;
; Print a string to the screen in TTY mode TODO: Use 0x10 AH 0x13H
;
; Inputs:
;   dl = character to print
[bits 16]
print_chr:
    pusha
    mov ah, 0x0E
    mov al, dl
    int 0x10
    popa
    ret

print_string:
    pusha
    mov ah, 0x0E ; Enable BIOS TTY
    mov cx, 0    ; Start at 0
    .loop:
        lodsb ; Load next byte from string
        test al, al ; Did we reach 0 byte (end of string)
        jz .done
        int 0x10 ; Print character
        jmp .loop

    .done:
        popa
        ret

print_string_nl:
    call print_string
    pusha
    mov ah, 0x0E
    mov al, 0x0A
    int 0x10
    mov al, 0x0D
    int 0x10
    popa
    ret
