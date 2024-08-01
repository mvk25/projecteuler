#include "util.h"
#include <stdint.h>

typedef unsigned __int128 uint128_t;

/** mod_power - function that performs modular exponential of numbers
 *
 * @base: our base
 * @exp: the exponent
 * @mod: our modulus
 *
 * Returns: b^e (mod p)
 */
uint128_t mod_power(uint128_t base, uint128_t exp, uint128_t mod) {
  uint128_t result = 1;
  int bin = 0;
  while (exp > 0) {
    if (exp % 2 == 1) {
      result = (base * result) % mod;
      bin = 1;
    } else {
      bin = 0;
    }
    //printf("bin %ld base %ld\n", bin, base);
    exp /= 2;
    base = (base * base) % mod;
  }
  return result;
}

//int main() {
  //printf("%d", mod_power(49, 5, 133));
//}

