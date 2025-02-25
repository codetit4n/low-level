#include "allocator.h"
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

// Example allocator
/**
 * Allocates a block of memory of (at least) `size` bytes.
 */
void *ealloc(size_t size) {
  size = align(size);
  printf("ealloc: Incomplete implementation!\n");
  exit(1);
}

void efree(void *ptr) {
  printf("efree: Not implemented!\n");
  exit(1);
}

/**
 * Memory alignment
 */
inline size_t align(size_t n) {
  return (n + sizeof(word_t) - 1) & ~(sizeof(word_t) - 1);
}
