global _start

extern check_long_mode
extern enable_paging
extern long_mode_start
extern setup_page_tables

section .text
bits 32
_start:
  call check_long_mode

  call setup_page_tables
  call enable_paging

  lgdt [gdt64.pointer]

  jmp gdt64.code:long_mode_start

section .rodata
gdt64:
    dq 0 ; zero entry
.code: equ $ - gdt64 ; new
    dq (1<<44) | (1<<47) | (1<<43) | (1<<53) ; code segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64
