#include "allocator.h"
#include <assert.h>
#include <stdlib.h>

int main() {
  // --------------------------------------
  // Test case 1: Alignment
  //
  // A request for 3 bytes is aligned to 8.
  //

  word_t *p1 = ealloc(3);
  MemoryBlock *p1mb = getHeader(p1);
  assert(p1mb->size == sizeof(word_t));

  // --------------------------------------
  // Test case 2: Exact amount of aligned bytes
  //
  word_t *p2 = ealloc(8);
  MemoryBlock *p2mb = getHeader(p2);
  assert(p2mb->size == 8);

  // --------------------------------------
  // Test case 3: Free the object
  //

  efree(p2);
  assert(p2mb->used == false);

  return EXIT_SUCCESS;
}
