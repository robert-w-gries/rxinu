global startup

extern check_cpuid
extern check_multiboot
extern init_sse
extern _start

section .text
bits 32
startup:
  mov esp, stack_top

  ; Save Multiboot information to register
  mov edi, ebx

  call check_multiboot
  call check_cpuid

  ; TODO: determine which of these init functions are necessary
  ; call init_fpu
  ; call init_pic
  ; call init_pit
  call init_sse

  jmp _start

section .bss
align 4096
stack_bottom:
  resb 4096 * 4
stack_top:
