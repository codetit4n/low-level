#include "allocator.h"
#include <stdio.h>
#include <stdlib.h> // For malloc, free

// example allocator
void *ealloc(size_t size) {
  void *ptr = malloc(size); // to be replaced with custom allocator
  return ptr;
}

void efree(void *ptr) {
  free(ptr); // to be replaced with custom allocator
}
