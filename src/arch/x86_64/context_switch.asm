# Credit to lachlansneff: https://github.com/nebulet/nebulet

.global x86_64_context_switch
.intel_syntax noprefix
# Context {
#   0x00: flags
#   0x08: rdi
#   0x10: rsi
#   0x18: rdx
#   0x20: rcx
#   0x28: r8
#   0x30: r9
#   0x38: rbx
#   0x40: r12
#   0x48: r13
#   0x50: r14
#   0x58: r15
#   0x60: rbp
#   0x68: rsp
# }
#
# rdi <- reference to previous `Context`
# rsi <- reference to next `Context`
x86_64_context_switch:
    # Save the previous context
    pushfq
    pop qword ptr [rdi] # save rflags into prev.flags
    mov [rdi+0x08], rdi # save rdi
    mov [rdi+0x10], rsi # save rsi
    mov [rdi+0x18], rdx # save rdx
    mov [rdi+0x20], rcx # save rcx
    mov [rdi+0x28], r8  # save r8
    mov [rdi+0x30], r9  # save r9
    mov [rdi+0x38], rbx # save rbx
    mov [rdi+0x40], r12 # save r12
    mov [rdi+0x48], r13 # save r13
    mov [rdi+0x50], r14 # save r14
    mov [rdi+0x58], r15 # save r15
    mov [rdi+0x60], rbp # save rbp

    # Swap the stack pointers
    mov [rdi+0x68], rsp # save rsp
    mov rsp, [rsi+0x68] # set rsp

    # Switch to the next context
    mov rbp, [rsi+0x60] # set rbp
    mov r15, [rsi+0x58] # set r15
    mov r14, [rsi+0x50] # set r14
    mov r13, [rsi+0x48] # set r13
    mov r12, [rsi+0x40] # set r12
    mov rbx, [rsi+0x38] # set rbx
    mov r9,  [rsi+0x30] # set r9
    mov r8,  [rsi+0x28] # set r8
    mov rcx, [rsi+0x20] # set rcx
    mov rdx, [rsi+0x18] # set rdx
    mov rdi, [rsi+0x08] # set rdi

    push [rsi] # set rflags
    popfq

    mov rsi, [rsi+0x10] # set rsi, overriding next context pointer

    # leap of faith
    ret
