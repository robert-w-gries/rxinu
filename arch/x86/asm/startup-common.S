# http://wiki.osdev.org/Bare_Bones

.intel_syntax noprefix
.code32

.extern check_cpuid
.extern check_multiboot
.extern init_sse

# The multiboot standard does not define the value of the stack pointer register
# (esp) and it is up to the kernel to provide a stack. This allocates room for a
# small stack by creating a symbol at the bottom of it, then allocating 16384
# bytes for it, and finally creating a symbol at the top. The stack grows
# downwards on x86. The stack is in its own section so it can be marked nobits,
# which means the kernel file is smaller because it does not contain an
# uninitialized stack. The stack on x86 must be 16-byte aligned according to the
# System V ABI standard and de-facto extensions. The compiler will assume the
# stack is properly aligned and failure to align the stack will result in
# undefined behavior.
.section .bss
.align 4096
stack_bottom:
.skip 4096 # one page
.skip 4096 # one page
.skip 4096 # one page
.skip 4096 # one page
.skip 4096 # one page
.skip 4096 # one page
stack_top:

.section .text
.global startup
.type startup, @function
startup:
  # The bootloader has loaded us into 32-bit protected mode on a x86
  # machine. Interrupts are disabled. Paging is disabled. The processor
  # state is as defined in the multiboot standard. The kernel has full
  # control of the CPU. The kernel can only make use of hardware features
  # and any code it provides as part of itself. There's no printf
  # function, unless the kernel provides its own <stdio.h> header and a
  # printf implementation. There are no security restrictions, no
  # safeguards, no debugging mechanisms, only what the kernel provides
  # itself. It has absolute and complete power over the
  # machine.

  # To set up a stack, we set the esp register to point to the top of our
  # stack (as it grows downwards on x86 systems). This is necessarily done
  # in assembly as languages such as C cannot function without a stack.
  lea esp, stack_top

  # Save Multiboot information to register
  mov edi, ebx

  call check_multiboot
  call check_cpuid

  # TODO: determine which of these init functions are necessary
  # call init_fpu
  # call init_pic
  # call init_pit
  call init_sse

  jmp _start
