.intel_syntax noprefix

.extern rust_main

.section .text
.global _start
_start:
  call rust_main
  hlt
