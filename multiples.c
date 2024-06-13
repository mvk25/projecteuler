#include <stdio.h>

/**
 * multiples - Finds the sum of all multiples of 3 and 5 below 1000.
 *
 * Return: some sum
 */
int multiples() {
	int init_sum, i;

	init_sum = 0;
	for (i = 0; i < 1000; ++i) {
		if (i % 3 == 0 || i % 5 == 0) {
			init_sum += i;
		}
	}

	return init_sum;
}

int main(void) {
	printf("%d", multiples());
	return 0;
}
