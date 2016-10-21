.intel_syntax noprefix

.extern check_long_mode
.extern enable_paging
.extern rust_main
.extern setup_page_tables

.section .text
.global _start
_start:
  call check_long_mode

  call setup_page_tables
  call enable_paging
  # TODO: Enable long mode, remove debugging 'OK', and call rust code
  #call rust_main
  mov dword ptr [0xb8000], 0x2f4b2f4f
  hlt
