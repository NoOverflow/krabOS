;
; KrabbOS boot loader
;
; This file is part of KrabbOS.
;
; Resolve and jump to the kernel entry point.
;
[bits 32]
[extern _start]
call _start
jmp $
