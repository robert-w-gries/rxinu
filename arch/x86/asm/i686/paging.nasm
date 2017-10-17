; Source: http://os.phil-opp.com/entering-longmode.html

global enable_paging
global setup_page_tables

section .text
bits 32
setup_page_tables:
  ; recursive map pd
  mov eax, page_directory
  or eax, 0b11 ; present + writable
  mov [page_directory + 1023 * 4], eax

  ; map first pd entry to pt table
  mov eax, page_table
  or eax, 0b11 ; present + writable
  mov [page_directory], eax

  ; map each pt entry to fill 4 MB
  mov ecx, 0  ; counter variable
.identity_paging:
  ; map ecx-th pd entry to a huge page that starts at address 4KB*ecx
  mov eax, 0x1000                     ; 4KB
  mul ecx                             ; start address of ecx-th page
  or eax, 0b11                        ; present + writable + huge
  mov [page_table + ecx * 4], eax ; map ecx-th entry

  inc ecx              ; increase counter
  cmp ecx, 1024        ; if counter == 512, the whole pd table is mapped
  jne .identity_paging ; else map the next entry

  ret

enable_paging:
  ; load pd to cr3 register (cpu uses this to access the page directory table)
  mov eax, page_directory
  mov cr3, eax

  ; enable PAE-flag in cr4 (Physical Address Extension)
  ;mov eax, cr4
  ;or eax, 1 << 5
  ;mov cr4, eax

  ; Enable PSE (4 MB pages)
  mov eax, cr4
  or eax, 0x10
  mov cr4, eax

  ; enable paging in the cr0 register
  mov eax, cr0
  or eax, 1 << 31
  mov cr0, eax

  ret

section .bss
align 4096
page_directory:
  resb 4096
page_table:
  resb 4096
