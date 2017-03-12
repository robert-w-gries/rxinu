.intel_syntax noprefix
.code64

.extern rust_main

.section .text
.global long_mode_start
long_mode_start:
  call rust_main
  hlt
