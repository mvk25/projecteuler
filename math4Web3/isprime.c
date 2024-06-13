#include "util.h"
/** is_prime - checks if a number provided is prime
 *
 * @a: the number we are checking, we can also set it to take
 * a random number
 *
 * Return: 1 if prime, else 0
 */

int is_prime(int a) {
  int i;

  if (a == 2) return (1);
  else if (a == 1 || a == 0) return (0);

  for (i = 3; i < a; i += 2) {
    if (a % i == 0) return (0);
  }
  return (1);
}


