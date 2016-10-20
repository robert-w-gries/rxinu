.intel_syntax noprefix

.set MAGIC,       0xe85250d6
.set ARCH,        0
.set HEADER_LEN,  header_end - header_start
.set CHECKSUM,    -(MAGIC + ARCH + HEADER_LEN)
.set TYPE,        0
.set FLAGS,       0
.set SIZE,        8

.section .multiboot_header
header_start:
.long MAGIC
.long ARCH
.long HEADER_LEN
.long CHECKSUM
.align 8
.short TYPE
.short FLAGS
.long SIZE
header_end:
