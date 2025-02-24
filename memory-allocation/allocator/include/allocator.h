#ifndef ALLOCATOR_H
#define ALLOCATOR_H

#include<stddef.h>

void *ealloc(size_t size);
void efree(void* ptr);

#endif
