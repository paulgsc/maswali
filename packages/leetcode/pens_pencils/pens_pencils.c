
#include <stdio.h>

int waysToBuyPensPencils(int total, int cost1, int cost2) {
	int ways = 0;

	for (int pens = 0; pens * cost1 <= total; pens++) {
		int remaining = total - pens * cost1;

		ways += (remaining / cost2) + 1;
	}

	return ways;
}

void run_tests() {
	int total1 = 20, cost1_1 = 10, cost2_1 = 5;
	printf("Test Case 1: %d (Expected: 9)\n", waysToBuyPensPencils(total1, cost1_1, cost2_1));

	int total2 = 5, cost1_2 = 10, cost2_2 = 10;
	printf("Test Case 2: %d (Expected: 1)\n", waysToBuyPensPencils(total2, cost1_2, cost2_2));
}

int main() {
	run_tests();
	return 0;
}

