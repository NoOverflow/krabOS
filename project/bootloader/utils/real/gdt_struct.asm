;
; KrabbOS utils -- gdt
;
; This file is part of KrabbOS.
;
; Setup the Global Descriptor Table
;
;
;
;
gdt_start:
    gdt_null: ; 8 bytes, never referenced by the processor, and should always contain no data.
        dd 0x0
        dd 0x0

    gdt_code: ; 8 bytes, Code Segment descriptor
        dw 0xFFFF
        dw 0x0
        db 0x0
        db 10011010b ; present, ring 0, executable, conforming, read/write
        db 11001111b ; granularity, 32-bit, no 64-bit
        db 0x0

    gdt_data: ; 8 bytes, Data Segment descriptor
        dw 0xFFFF
        dw 0x0
        db 0x0
        db 10010010b ; present, ring 0, read/write
        db 11001111b ; granularity, 32-bit, no 64-bit
        db 0x0

gdt_end:

; GDT descriptor
; 6 bytes
; 0-1: size of the GDT
; 2-5: start address of the GDT
gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start
