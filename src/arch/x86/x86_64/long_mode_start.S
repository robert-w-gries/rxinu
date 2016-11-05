.intel_syntax noprefix

.extern rust_main

.section .text
.global long_mode_start
long_mode_start:
  call rust_main
  hlt
