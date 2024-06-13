#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

// Function to check if a number is prime
bool is_prime(int num) {
  if (num <= 1) return false;
  if (num <= 3) return true;
  if (num % 2 == 0 || num % 3 == 0) return false;
  for (int i = 5; i * i <= num; i += 6) {
    if (num % i == 0 || num % (i + 2) == 0) return false;
  }
  return true;
}

// Function to find the prime factors of a number
int *prime_factors(int a) {
  int i;
  int items = 0;
  int *ptr = NULL;
  
  for (i = 2; i <= a; ++i) {
    while (is_prime(i) && a % i == 0) {
      a /= i;
      items++;
      ptr = realloc(ptr, sizeof(int) * (items + 1)); // Allocate extra space for the terminating 0
      if (ptr == NULL) {
        perror("realloc failed");
        exit(EXIT_FAILURE);
      }
      ptr[items - 1] = i;
    }
  }
  if (ptr != NULL) {
    ptr[items] = 0; // Null-terminate the array with 0
  }
  return ptr;
}

int main() {
  int i;
  int *numptrs = prime_factors(1000);
  if (numptrs == NULL) {
    fprintf(stderr, "Error allocating memory for prime factors.\n");
    return 1;
  }

  // Print prime factors
  for (i = 0; numptrs[i] != 0; ++i) {
    if (i == 0) {
      printf("%d", numptrs[0]);
    } else {
      printf(" * %d", numptrs[i]);
    }
  }
  printf("\n");

  // Free the allocated memory
  free(numptrs);

  return 0;
}

