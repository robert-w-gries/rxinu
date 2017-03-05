# Source: http://os.phil-opp.com/entering-longmode.html

.intel_syntax noprefix
.code32

.section .bss
.align 4096
p4_table:
.skip 4096
p3_table:
.skip 4096
p2_table:
.skip 4096

.section .text
.global setup_page_tables
.type setup_page_tables, @function
setup_page_tables:
  # map first P4 entry to P3 table
  lea eax, p3_table
  or eax, 0b11 # present + writable
  mov [p4_table], eax

  # map first P3 entry to P2 table
  lea eax, p2_table
  or eax, 0b11 # present + writable
  mov [p3_table], eax

  # map each P2 entry to a huge 2MiB page
  mov ecx, 0  # counter variable
.map_p2_table:
  # map ecx-th P2 entry to a huge page that starts at address 2MiB*ecx
  mov eax, 0x200000  # 2MiB
  mul ecx            # start address of ecx-th page
  or eax, 0b10000011 # present + writable + huge
  mov [p2_table + ecx * 8], eax # map ecx-th entry

  inc ecx            # increase counter
  cmp ecx, 512       # if counter == 512, the whole P2 table is mapped
  jne .map_p2_table  # else map the next entry

  ret

.global enable_paging
.type enable_paging, @function
enable_paging:
  # load P4 to cr3 register (cpu uses this to access the P4 table)
  mov eax, p4_table
  mov cr3, eax

  # enable PAE-flag in cr4 (Physical Address Extension)
  mov eax, cr4
  or eax, 1 << 5
  mov cr4, eax

  # set the long mode bit in the EFER MSR (model specific register)
  mov ecx, 0xC0000080
  rdmsr
  or eax, 1 << 8
  wrmsr

  # enable paging in the cr0 register
  mov eax, cr0
  or eax, 1 << 31
  mov cr0, eax

  ret
