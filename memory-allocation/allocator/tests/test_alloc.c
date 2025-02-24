#include "allocator.h"
#include <stdio.h>
#include <stdlib.h>

#define TEST_ASSERT(cond, msg)                                                 \
  if (!(cond)) {                                                               \
    printf("TEST FAILED: %s\n", msg);                                          \
    return EXIT_FAILURE;                                                       \
  }

int main() {
  printf("Running allocator tests...\n");

  void *ptr = ealloc(256);
  TEST_ASSERT(ptr != NULL, "ealloc(256) should return a valid pointer");

  efree(ptr);
  printf("TEST PASSED: Memory allocation and deallocation successful\n");

  return EXIT_SUCCESS; // CTest recognizes EXIT_SUCCESS as a pass
}
