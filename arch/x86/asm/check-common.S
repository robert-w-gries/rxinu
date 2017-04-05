# http://os.phil-opp.com/entering-longmode.html

.intel_syntax noprefix
.code32

.section .text
.global check_multiboot
.type check_multiboot, @function
check_multiboot:
  cmp eax, 0x36d76289
  jne .no_multiboot
  ret
.no_multiboot:
  mov al, 0
  jmp error

.global check_cpuid
.type check_cpuid, @function
check_cpuid:
  # Check if CPUID is supported by attempting to flip the ID bit (bit 21)
  # in the FLAGS register. If we can flip it, CPUID is available.

  # Copy FLAGS in to EAX via stack
  pushfd
  pop eax

  # Copy to ECX as well for comparing later on
  mov ecx, eax

  # Flip the ID bit
  xor eax, 1 << 21

  # Copy EAX to FLAGS via the stack
  push eax
  popfd

  # Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
  pushfd
  pop eax

  # Restore FLAGS from the old version stored in ECX (i.e. flipping the
  # ID bit back if it was ever flipped).
  push ecx
  popfd

  # Compare EAX and ECX. If they are equal then that means the bit
  # wasn't flipped, and CPUID isn't supported.
  cmp eax, ecx
  je .no_cpuid
  ret
.no_cpuid:
  mov al, 1
  jmp error
