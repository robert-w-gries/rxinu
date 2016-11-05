.intel_syntax noprefix
.code32

.extern check_long_mode
.extern enable_paging
.extern long_mode_start
.extern setup_page_tables

.section .text
.global _start
_start:
  call check_long_mode

  call setup_page_tables
  call enable_paging

flush_gdt:
  lgdt [gdtr]
  jmp 0x08:complete_flush

complete_flush:
  mov eax, 0x10

  mov ds, eax
  mov es, eax
  mov fs, eax
  mov gs, eax
  mov ss, eax

  jmp long_mode_start

.section .rodata
.align 16
gdtr:
.word gdt_end - gdt - 1
.quad gdt
gdt:
.quad 0 # zero entry
.quad 0x209A0000000000 # code segment
.quad 0x920000000000   # data segment
gdt_end:
