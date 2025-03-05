#include "allocator.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
  // --------------------------------------
  // Test case 1: Alignment
  //
  // A request for 3 bytes is aligned to 8.
  //

  // word_t *p1 = ealloc(3);
  // MemoryBlock *p1mb = getHeader(p1);
  // assert(p1mb->size == sizeof(word_t));

  //// --------------------------------------
  //// Test case 2: Exact amount of aligned bytes
  ////
  // word_t *p2 = ealloc(8);
  // MemoryBlock *p2mb = getHeader(p2);
  // assert(p2mb->size == 8);

  //// --------------------------------------
  //// Test case 3: Free the object
  ////

  // efree(p2);
  // assert(p2mb->used == false);

  //// --------------------------------------
  //// Test case 4: The block is reused
  ////
  //// A consequent allocation of the same size reuses
  //// the previously freed block.
  ////

  // word_t *p3 = ealloc(8);
  // MemoryBlock *p3mb = getHeader(p3);
  // assert(p3mb->size == 8);
  // assert(p3mb == p2mb); // Reused!

  // Init the heap, and the searching algorithm.
  init(NextFit);

  // --------------------------------------
  // Test case 5: Next search start position
  //

  // [[8, 1], [8, 1], [8, 1]]
  ealloc(8);
  ealloc(8);
  ealloc(8);

  // [[8, 1], [8, 1], [8, 1], [16, 1], [16, 1]]
  word_t *o1 = ealloc(16);
  word_t *o2 = ealloc(16);

  // [[8, 1], [8, 1], [8, 1], [16, 0], [16, 0]]
  efree(o1);
  efree(o2);

  printf("here\n");

  // [[8, 1], [8, 1], [8, 1], [16, 1], [16, 0]]
  word_t *o3 = ealloc(16);

  printf(" now here\n %p\n", srchStart);
  printf(" now here\n %p\n", getHeader(o3));
  // Start position from o3:
  // assert(srchStart == getHeader(o3));

  //// [[8, 1], [8, 1], [8, 1], [16, 1], [16, 1]]
  ////                           ^ start here
  // ealloc(16);

  return EXIT_SUCCESS;
}
