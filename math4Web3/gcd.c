#include <stdio.h>
#include <stdlib.h>

/**
 * gcd - returns the gcd of two values
 *
 * @m: first value
 * @n: second value
 *
 * Return: the gcd
 */
int gcd(int m, int n)
{
	if ((m % n) == 0)
	{
		return (n);
	}
	else {
		return (gcd(n, m % n));
	}
}
