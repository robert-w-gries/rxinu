# Source: http://os.phil-opp.com/entering-longmode.html

.intel_syntax noprefix
.code32

.section .bss
.align 4096
page_directory:
.skip 4096
page_table:
.skip 4096

.section .text
.global setup_page_tables
.type setup_page_tables, @function
setup_page_tables:
  # map first pd entry to pt
  lea eax, page_table
  or eax, 0b11 # present + writable
  mov [page_directory], eax

  # map each pd entry to a huge 4MiB page
  mov ecx, 0  # counter variable
.identity_paging:
  # map ecx-th pd entry to a huge page that starts at address 4MiB*ecx
  mov eax, 0x400000                   # 4MiB
  mul ecx                             # start address of ecx-th page
  or eax, 0b10000011                  # present + writable + huge
  mov [page_directory + ecx * 4], eax # map ecx-th entry

  inc ecx             # increase counter
  cmp ecx, 256        # if counter == 256, the whole pd table is mapped
  jne .identity_paging # else map the next entry

  ret

.global enable_paging
.type enable_paging, @function
enable_paging:
  # load pd to cr3 register (cpu uses this to access the page directory table)
  lea eax, page_directory
  mov cr3, eax

  # enable PSE (4 MiB pages)
  mov eax, cr4
  or eax, 0x10
  mov cr4, eax

  # enable paging in the cr0 register
  mov eax, cr0
  or eax, 1 << 31
  mov cr0, eax

  ret
