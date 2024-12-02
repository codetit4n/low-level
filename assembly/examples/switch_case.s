.data
.intel_syntax noprefix

# exit(code)
.set SYS_EXIT, 60
.set EXIT_CODE, 0

# write (fd, buf_adr, buf_len)
.set SYS_WRITE, 1
.set STDOUT, 1

# read(fd, buf_adr, buf_len)
.set SYS_READ, 0
.set STDIN, 0

# ASCII code for lowercase 'a'
.set ASCII_A, 97

# e.g.
# 'A' + 32 = 'a'
# 'a' - 32 = 'A'
.set CASE_DIFF, 32

# single byte in memory
CHAR:
	.byte 0

	.text

	.global _start

_start:
# read(STDIN, CHAR, 1)
mov rax, SYS_READ # System call number for read
mov rdi, STDIN # File descriptor for standard input
lea rsi, CHAR # Load address of CHAR into RSI
mov rdx, 1 # Number of bytes to read
syscall

mov al, [CHAR] # Load byte at CHAR into AL register
cmp al, ASCII_A # Compare AL with ASCII_A (lowercase 'a')

jge MAKE_UPPERCASE  # If AL >= 'a', convert to uppercase

MAKE_LOWERCASE:
	add al, CASE_DIFF # Convert to lowercase by adding CASE_DIFF
	jmp STORE_CHAR # Skip to storing the result

MAKE_UPPERCASE:
	sub al, CASE_DIFF # Convert to uppercase by subtracting CASE_DIFF

STORE_CHAR:
	mov [CHAR], al # Store the result back into CHAR

WRITE:
# write(STDOUT, CHAR, 1)
mov rax, SYS_WRITE # System call number for write
mov rdi, STDOUT # File descriptor for standard output
lea rsi, CHAR # Load address of CHAR into RSI
mov rdx, 1 # Number of bytes to write
syscall

# exit(EXIT_CODE)
mov rax, SYS_EXIT # System call number for exit
mov rdi, EXIT_CODE # Exit code
syscall
