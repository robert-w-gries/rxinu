.intel_syntax noprefix

.extern rust_main
.extern check_long_mode

.section .text
.global _start
_start:
  call check_long_mode
  #call rust_main
mov dword ptr [0xb8000], 0x2f4b2f4f
  hlt
