#ifndef ALLOCATOR_H
#define ALLOCATOR_H

#include<stddef.h>
#include <stdbool.h>
#include <stdint.h>


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

word_t *ealloc(size_t size);

void efree(word_t *ptr);


size_t align(size_t n);

size_t allocSize(size_t size);

MemoryBlock *getHeader(word_t *data);

#endif
