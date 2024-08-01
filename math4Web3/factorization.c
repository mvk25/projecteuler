#include <stdio.h>
#include <stdlib.h>
#include "util.h"
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
  int *factors = prime_factors(31500);
  int i;

  for (i = 0; factors[i] != 0; i++) {
    printf("%d ", factors[i]);
  }
  return (0);
}
