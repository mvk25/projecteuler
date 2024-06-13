#include <stdio.h>
#include "util.h"

#define BUFSIZE 16

/** phi - A program that calculates the phi of a number
 *
 * @int: It takes an integer
 *
 * Return - Returns an int, the phi of the parameter
 */

int phi(int a) {
  int i;
  int set[BUFSIZE] = {0};
  int *numptrs = prime_factors(a);
  int pos = 0;
  double phi = a;

  if (is_prime(a)) return (a - 1);
  for (i = 1; numptrs[i] != 0; ++i) {
    if (numptrs[i - 1] != numptrs[i]) {
      set[pos++] = numptrs[i - 1];
    } 
  }
  set[pos++] = numptrs[i - 1];

  free(numptrs);
  for (i = 0; set[i] != 0; ++i) {
    phi *= (1.0 - (1.0 / set[i]));
  }
  return phi;
}
