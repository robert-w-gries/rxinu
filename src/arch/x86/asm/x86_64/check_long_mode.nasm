global check_long_mode
extern error

section .text
bits 32

; Throw error 2 if the CPU doesn't support Long Mode.
check_long_mode:
  ; test if extended processor info in available
  mov eax, 0x80000000    ; implicit argument for cpuid
  cpuid                  ; get highest supported argument
  cmp eax, 0x80000001    ; it needs to be at least 0x80000001
  jb .no_long_mode       ; if it's less, the CPU is too old for long mode

  ; use extended info to test if long mode is available
  mov eax, 0x80000001    ; argument for extended processor info
  cpuid                  ; returns various feature bits in ecx and edx
  test edx, 1 << 29      ; test if the LM-bit is set in the D-register
  jz .no_long_mode       ; If it's not set, there is no long mode
  ret
.no_long_mode:
  mov al, "2"
  jmp error
