# Credit to lachlansneff: https://github.com/nebulet/nebulet

.global x86_64_context_switch
.intel_syntax noprefix
# Context {
#   0x0: flags
#   0x8: rbx
#   0x10: r12
#   0x18: r13
#   0x20: r14
#   0x28: r15
#   0x30: rbp
#   0x38: rsp
# }
#
# rdi <- reference to previous `Context`
# rsi <- reference to next `Context`
x86_64_context_switch:
    # Save the previous context
    pushfq
    pop qword ptr [rdi] # save rflags into prev.flags
    # Rust inline assembly error: invalid operand
    #mov [rdi+0x8], cr3  # save rbx
    mov [rdi+0x08], rbx  # save rbx
    mov [rdi+0x10], r12 # save r12
    mov [rdi+0x18], r13 # save r13
    mov [rdi+0x20], r14 # save r14
    mov [rdi+0x28], r15 # save r15
    mov [rdi+0x30], rbp # save rbp

    # Swap the stack pointers
    mov [rdi+0x38], rsp # save rsp
    mov rsp, [rsi+0x38] # set rsp

    # Switch to the next context
    mov rbp, [rsi+0x30] # set rbp
    mov r15, [rsi+0x28] # set r15
    mov r14, [rsi+0x20] # set r14
    mov r13, [rsi+0x18] # set r13
    mov r12, [rsi+0x10] # set r12
    mov rbx, [rsi+0x08]  # set rbx
    # Rust inline assembly error: invalid operand
    #mov cr3, [rsi+0x8]  # set rbx
    push [rsi] # set rflags
    popfq

    # leap of faith
    ret
