#include "allocator.h"
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

typedef intptr_t word_t;

typedef struct MemoryBlock MemoryBlock;

// Linked list for blocks of memory
struct MemoryBlock {
  // --------------------
  // 1. Object header
  size_t size;
  bool used;
  MemoryBlock *next;
  // --------------------
  // 2. User data
  // payload pointer
  word_t data[1];
};

// Tracking the start and end (the top) of the heap
static MemoryBlock *heapStart = NULL;
static MemoryBlock *top = NULL;

/**
 * Requests (maps) memory from OS.
 */
MemoryBlock *requestFromOS(size_t size) {
  // Find the current heap brk
  MemoryBlock *block = sbrk(0);

  // Out of memory
  if (sbrk(allocSize(size)) == (void *)-1) {
    return NULL;
  }

  return block;
}

// Example allocator
/**
 * Allocates a block of memory of (at least) `size` bytes.
 */
void *ealloc(size_t size) {
  size = align(size);

  MemoryBlock *block = requestFromOS(size);

  block->size = size;
  block->used = true;

  // Init heap
  if (heapStart == NULL) {
    heapStart = block;
  }

  // Chain the blocks
  if (top != NULL) {
    top->next = block;
  }

  top = block;

  return block->data;
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

/**
 * Returns total allocation size, reserving in addition the space for
 * the Block structure (object header + first data word).
 *
 * Since the `word_t data[1]` already allocates one word inside the Block
 * structure, we decrease it from the size request: if a user allocates
 * only one word, it's fully in the Block struct.
 */
inline size_t allocSize(size_t size) {
  return size + sizeof(MemoryBlock) - sizeof(((MemoryBlock *)0)->data);
}
