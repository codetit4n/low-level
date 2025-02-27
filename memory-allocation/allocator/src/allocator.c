#include "allocator.h"
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

MemoryBlock *findBlock(size_t size);

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
  if (sbrk(allocSize(size)) ==
      (void *)-1) { // (void *)-1 - means invalid pointer
    return NULL;
  }

  return block;
}

// Example allocator
/**
 * Allocates a block of memory of (at least) `size` bytes.
 */
word_t *ealloc(size_t size) {
  size = align(size);

  // 1. Search for an available free block:
  MemoryBlock *block = findBlock(size);
  if (block) {
    return block->data;
  }

  // 2. If block not found in the free list, request from OS:

  block = requestFromOS(size);

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

MemoryBlock *findBlock(size_t size) {
  MemoryBlock *block = requestFromOS(size);
  return block;
}

/**
 * Returns the object header
 */
MemoryBlock *getHeader(word_t *data) {
  return (MemoryBlock *)(((char *)data + sizeof(((MemoryBlock *)0)->data)) -
                         sizeof(MemoryBlock));
}

void efree(word_t *ptr) {
  MemoryBlock *block = getHeader(ptr);
  block->used = false;
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
