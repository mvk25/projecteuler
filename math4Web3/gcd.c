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

int main(int argc __attribute__((unused)), char **argv)
{
	int paramA = atoi(argv[1]);
	int paramB = atoi(argv[2]);
	printf("GCD between %d and %d: %d\n", paramA, paramB, gcd(paramA, paramB));
	return (0);
}
