# memory allocation

Learning memory allocation in C using `malloc` and `free`.

### Some useful tools:

- compile: `gcc test_malloc.c -o test_malloc`
- `strace` - `strace -e brk,mmap,munmap ./test_malloc`
  see system calls like:
  - `brk()` - heap expansion for small allocations
  - `mmap()` - for large allocations
  - `munmap()` - dellocations
- `valgrind` - `valgrind --leak-check=full ./test_malloc`
  Find any potential memory leaks
