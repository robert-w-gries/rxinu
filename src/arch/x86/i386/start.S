.intel_syntax noprefix
.code32

.extern rust_main

.section .text
.global _start
_start:
  # Pass Multiboot info pointer to main function
  push edi

  call rust_main
  hlt
