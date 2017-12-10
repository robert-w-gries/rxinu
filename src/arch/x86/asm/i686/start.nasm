global _start

extern enable_paging
extern rust_main
extern setup_page_tables

section .text
bits 32
_start:
  call setup_page_tables
  call enable_paging

  ; Pass Multiboot info pointer to main function
  push edi

  call rust_main
.os_returned
  cli
  hlt
