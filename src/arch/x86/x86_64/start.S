.intel_syntax noprefix

.extern rust_main

.section .text
.global _start
_start:
  #call rust_main
mov dword ptr [0xb8000], 0x2f4b2f4f
  hlt
