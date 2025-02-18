#include <stdio.h>
#include <stdlib.h>

int main() {
  int *arr = (int *)malloc(10 * sizeof(int));

  if (!arr) {
    printf("memory allocation failed!");
    return 1;
  }

  printf("memory allocation at: %p\n", arr);
  //  free(arr);
  printf("memory freed.\n");

  return 0;
}
