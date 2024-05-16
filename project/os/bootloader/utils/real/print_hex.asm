;
; KrabbOS utils -- disk_read
;
; This file is part of KrabbOS.
;
; Print a hexadecimal number to the screen in TTY mode TODO: Use 0x10 AH 0x13H
;
; Inputs:
;  DX = Number to print
;
[bits 16]
_print_hex_digit:
    cmp cx, 0xA
    jge ._let_hex
    ._dig_hex:
        add cl, '0'
        jmp ._print_hex_digit
    ._let_hex:
        add cl, 'A' - 0xA
        jmp ._print_hex_digit
    ._print_hex_digit:
        mov al, cl
        int 0x10
        ret

print_hex:
    pusha
    mov ah, 0x0E ; Enable BIOS TTY
    mov al, '0'
    int 0x10
    mov al, 'x'
    int 0x10

    ; Fourth nibble
    mov cx, dx
    and cx, 0xF000
    shr cx, 12
    call _print_hex_digit

    ; Third nibble
    mov cx, dx
    and cx, 0x0F00
    shr cx, 8
    call _print_hex_digit

    ; Second nibble
    mov cx, dx
    and cx, 0x00F0
    shr cx, 4
    call _print_hex_digit

    ; First nibble
    mov cx, dx
    and cx, 0x000F
    call _print_hex_digit

    popa
    ret

print_hex_nl:
    call print_hex
    pusha
    mov ah, 0x0E
    mov al, 0x0A
    int 0x10
    mov al, 0x0D
    int 0x10
    popa
    ret
