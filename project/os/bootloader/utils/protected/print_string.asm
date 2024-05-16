[bits 32] ; using 32-bit protected mode

; this is how constants are defined
VIDEO_MEM_TOP_LEFT equ 0xB8000 ; (0x000A0000-0x000BFFFF)
WHITE_ON_BLACK equ 0x0F

print_string_pm:
    pusha
    mov edx, VIDEO_MEM_TOP_LEFT

print_string_pm_loop:
    mov al, [ebx]
    mov ah, WHITE_ON_BLACK

    cmp al, 0
    je print_string_pm_done

    mov [edx], ax
    inc ebx
    add edx, 2

    jmp print_string_pm_loop

print_string_pm_done:
    popa
    ret
