
#include <stdio.h>
#include <stdlib.h>

int get_max_charge(int *chargeTimes, int start, int end) {
	int max_charge = chargeTimes[start];
	for (int i = start + 1;  i <= end; i++) {
		if (chargeTimes[i] > max_charge) {
			max_charge = chargeTimes[i];
		}
	}
	return max_charge;
}

long long get_running_sum(int *runningCosts, int start, int end) {
	long long sum = 0;
	for (int i = start; i <= end; i++) {
		sum += runningCosts[i];
	}
	return sum;
}

int maxConsecutiveRobots(int* chargeTimes, int* runningCosts, int n, long long budget) {
	int max_consecutive = 0;
	int left = 0;
	long long running_sum = 0;

	for (int right = 0; right < n; right++) {
		running_sum += runningCosts[right];

		int max_charge = get_max_charge(chargeTimes, left, right);
		long long total_cost = max_charge + (right - left + 1) * running_sum;

		while (total_cost > budget) {
			running_sum -= runningCosts[left];
			left++;
			max_charge = get_max_charge(chargeTimes, left, right);
			total_cost = max_charge + (right - left + 1) * running_sum;
		}

		int current_window_size = right - left + 1;
		if (current_window_size > max_consecutive) {
			max_consecutive = current_window_size;
		}
	}

	return max_consecutive;
}

void run_tests() {
	int chargeTimes1[] = {3, 6, 1, 3, 4};
	int runningCosts1[] = {2, 1, 3, 4, 5};
	int n1 = 5;
	long long budget1 = 25;
	printf("Test Case 1: %d (Expected: 3)\n", maxConsecutiveRobots(chargeTimes1, runningCosts1, n1, budget1));


	int chargeTimes2[] = {11, 12, 19};
	int runningCosts2[] = {10, 8, 7};
	int n2 = 3;
	long long budget2 = 19;
	printf("Test Case 2: %d (Expected: 0)\n", maxConsecutiveRobots(chargeTimes2, runningCosts2, n2, budget2));


	int chargeTimes3[] = {1, 3, 5, 7};
	int runningCosts3[] = {1, 1, 1, 1};
	int n3 = 4;
	long long budget3 = 10;
	printf("Test Case 3: %d (Expected: 4)\n", maxConsecutiveRobots(chargeTimes3, runningCosts3, n3, budget3));

	int chargeTimes4[] = {3, 6, 8, 10, 12};
	int runningCosts4[] = {1, 1, 1, 1, 1};
	int n4 = 5;
	long long budget4 = 30;
	printf("Test Case 4: %d (Expected: 5)\n", maxConsecutiveRobots(chargeTimes4, runningCosts4, n4, budget4));
}


int main() {
	run_tests();
	return 0;
}

