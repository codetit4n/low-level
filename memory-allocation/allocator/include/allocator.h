#ifndef ALLOCATOR_H
#define ALLOCATOR_H

#include<stddef.h>
#include <stdbool.h>
#include <stdint.h>

typedef intptr_t word_t;

// Linked list for blocks of memory
struct MemoryBlock {
  // --------------------
  // 1. Object header
  size_t size;
  bool used;
  struct MemoryBlock *next;
  // --------------------
  // 2. User data
  // payload pointer
  word_t data[1];
};

// Tracking the start and end (the top) of the heap
static struct MemoryBlock *heapStart = NULL;
static struct MemoryBlock *top = NULL;

void *ealloc(size_t size);
void efree(void* ptr);
size_t align(size_t n);

#endif
