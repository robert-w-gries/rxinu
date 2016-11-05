.intel_syntax noprefix
.code32

.global error
error:
  mov dword ptr [0xb8000], 0x4f524f45
  mov dword ptr [0xb8004], 0x4f3a4f52
  mov dword ptr [0xb8008], 0x4f204f20
  mov byte      [0xb800a], al
  hlt
