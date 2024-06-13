#ifndef UTIL_H
#define UTIL_H
#include <stdlib.h>
#include <stdio.h>
typedef unsigned __int128 uint128_t;


int is_prime(int a);
int *prime_factors(int a);
int phi(int a);
uint128_t mod_power(uint128_t base, uint128_t exp, uint128_t mod);

#endif /* UTIL_H */
