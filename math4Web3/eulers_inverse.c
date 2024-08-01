#include "util.h"

/**
 *  eulers_inverse - this function calculates the inverse using eulers
 *  theorom
 *
 *  @e: our base input
 *  @p: our modulus
 *
 *  Return - returns d(decrypt key in RSA or sth)
 */
int eulers_inverse(int e, int p) {
  int eulers_phi = phi(p);
  int exp = eulers_phi - 1;

  int result = mod_power(e, exp, p);
  return result;
}

//int main() {
  //int inverse = eulers_inverse(317, 14863);
  //printf("%ld\n", inverse);
  //return (0);
//}
