# http://wiki.osdev.org/SSE

.intel_syntax noprefix
.code32

.section .text
.global init_sse
.type init_sse, @function
# Check for SSE and enable it
init_sse:
  # check for SSE
  mov eax, 0x1
  cpuid
  test edx, 1<<25
  jz .no_sse

  # enable SSE
  mov eax, cr0
  and ax, 0xFFFB  #clear coprocessor emulation CR0.EM
  or ax, 0x2      #set coprocessor monitoring  CR0.MP
  mov cr0, eax
  mov eax, cr4
  or ax, 3 << 9   #set CR4.OSFXSR and CR4.OSXMMEXCPT at the same time
  mov cr4, eax

  ret
.no_sse:
  mov al, 2
  jmp error
