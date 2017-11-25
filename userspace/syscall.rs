pub fn kill(pid: usize, sig: usize) -> Result<usize> {
    syscall!(KILL, pid, sig)
}
