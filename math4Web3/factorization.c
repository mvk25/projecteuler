#include <stdio.h>
#include <stdlib.h>
#include "util.h"
#include <time.h>
/** prime_factors - calculates the prime factorization of the input
 *
 * a: the input type
 *
 * Return - returns the a malloc of the ints or sth
 */

int *prime_factors(int a) {
  int i;
  int items = 0;
  int *ptr = NULL;
  for (i = 2; i <= a; ++i) {
    if (is_prime(i) && a % i == 0) {
      a /= i;
      items++;
      ptr = realloc(ptr, sizeof(int) * (items + 1));
      if (ptr == NULL) {
        perror("realloc failed");
        exit(EXIT_FAILURE);
      }
      ptr[items - 1]  = i;
      --i;
    }
  }

  if (ptr != NULL) ptr[items] = 0;
  return ptr;
}

int main() {
  clock_t start, end;
  double cpu_time_used;
  start = clock();
  int *numptrs = prime_factors(756780);

  for (int i = 0; numptrs[i] != 0; ++i) {
    printf("%i  ", numptrs[i]);
  }

  free(numptrs);
  end = clock();

  cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
  printf("Time taken: %f seconds\n", cpu_time_used);
  return (0);
}
