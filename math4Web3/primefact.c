#include <stdio.h>

/**
 * main - prints largest prime factor.
 * Return: Always 0.
 */

int main(void)
{
	long int n, fp;

	/*n = 612852475143;*/
	n = 500;
	for (fp = 2; fp <= n; fp++)
	{
		if (n % fp == 0)
		{
			printf("%li ", fp);
			n /= fp;
			fp--;
		}
	}
	printf("%ld\n", fp);
	return (0);
}
