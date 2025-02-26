#ifndef ALLOCATOR_H
#define ALLOCATOR_H

#include<stddef.h>
#include <stdbool.h>
#include <stdint.h>


void *ealloc(size_t size);
void efree(void* ptr);
size_t align(size_t n);
size_t allocSize(size_t size);

#endif
