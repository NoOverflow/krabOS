;
; KrabbOS utils -- disk_read
;
; This file is part of KrabbOS.
;
; Use bios function 0x13 to read a sector from disk.
;
; Inputs:
;  DL = drive number
;  CH = cylinder number
;  CL = sector number
;  DH = head number
;  ES:BX = buffer to read to
;

[bits 16]
; Constants
BIOS_DISK_OPS_INT EQU 0x13
BIOS_DISK_OPS_INT_READ EQU 0x2
KRB_OK EQU 0x0
KRB_ERROR EQU 0x1

read_disk:
    pusha
    push ax

    mov ah, BIOS_DISK_OPS_INT_READ
    int BIOS_DISK_OPS_INT
    jc read_disk_error

    pop bx ; Restore al to ah for error checking (contains the requested amount of sectors to read)
    mov ah, bl
    cmp al, ah
    jne read_sector_error ; If the amount of sectors read is different from the requested, return error
    mov si, SECTOR_READ_SUCCESS_STR
    call print_string
    mov dl, al
    call print_hex_nl
    popa
    mov ax, KRB_OK

    read_disk_end:
        ret

    read_disk_error:
        add sp, 0x2 ; Remove the pushed al
        popa
        mov al, KRB_ERROR
        jmp read_disk_end

    read_sector_error:
        mov si, SECTOR_READ_ERROR_STR
        call print_string
        mov dl, al
        call print_hex
        mov dl, ':'
        call print_chr
        mov dl, ah
        call print_hex_nl
        popa
        mov al, KRB_ERROR
        jmp read_disk_end

SECTOR_READ_SUCCESS_STR db "Success. read sectors ", 0
SECTOR_READ_ERROR_STR db "Read err (e:g) ", 0
