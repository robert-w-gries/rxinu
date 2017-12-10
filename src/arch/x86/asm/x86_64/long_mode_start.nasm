; Copyright 2016 Philipp Oppermann. See the README.md
; file at the top-level directory of this distribution.
;
; Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
; http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
; <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
; option. This file may not be copied, modified, or distributed
; except according to those terms.

global long_mode_start
extern rust_main

section .text
bits 64
long_mode_start:
  ; load 0 into all data segment registers
  mov ax, 0
  mov ss, ax
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax

  ; call rust main (with multiboot pointer in rdi)
  call rust_main
.os_returned:
  cli
  hlt
