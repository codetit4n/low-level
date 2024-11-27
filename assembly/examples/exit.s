# This program exits gracefully with exit code 0
.data
.intel_syntax noprefix

.set SYS_EXIT, 60 # syscall number for exit
.set EXIT_CODE, 0 # exit code - 0 for graceful exit, non-zero for error

.text

.global _start

_start:
# exit(code)
mov rax, SYS_EXIT
mov rdi, EXIT_CODE
syscall
