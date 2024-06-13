#include <stdio.h>
#include <stdint.h>

/**
 * fibonacci - finds the fibonacci of an input num
 *
 * @num: Our input
 *
 * Return: Fibonacci of that number at that term
 */
uint64_t fibonacci(uint32_t num) {
	if (num == 0)
		return 0;
	else if (num == 1)
		return 1;
	else return fibonacci(num - 1) + fibonacci(num -2);
}

/**
 * even_fibonacci - find the sum of even values terms whose values do not exceed four million
 *
 * @limit: uint64_t
 *
 * Return: sum of even valued terms
 */
uint64_t even_fibonacci(uint32_t limit) {
	uint64_t even_sum = 0;
	uint32_t i = 2;
	uint64_t fib;
	while (fib < limit) {
		fib = fibonacci(i);
		if (fib % 2 == 0) {
			even_sum += fib;
		}
		i += 1;
	}

	return even_sum;
}


int main(void) {
	printf("%ld", even_fibonacci(4000000));
	return 0;
}
