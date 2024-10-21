
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#define MAX_LENGTH 50001

void computeLPS(const char* pattern, int* lps, int len) {
	int len_prefix = 0;
	lps[0] = 0;
	int i = 1;
	
	while (i < len) {
		if (pattern[i] == pattern[len_prefix]) {
			len_prefix++;
			lps[i] = len_prefix;
			i++;
		} else {
			if (len_prefix != 0) {
				len_prefix = lps[len_prefix - 1];
			} else {
				lps[i] = 0;
				i++;
			}
		}
	}
}

char* shortestPalindrome(const char* s) {
	int len = strlen(s);
	if (len <= 1) return strdup(s);

	char* reversed = malloc(len + 1);
	for (int i = 0; i < len; i++) {
		reversed[i] = s[len - 1 - i];
	}
	reversed[len] = '\0';

	char* concat = malloc(2 * len + 2);
	sprintf(concat, "%s#%s", s, reversed);

	int concat_len = 2 * len + 1;
	int* lps = malloc(concat_len * sizeof(int));
	computeLPS(concat, lps, concat_len);

	int prefix_len = lps[concat_len - 1];
	char* result = malloc(2 * len - prefix_len + 1);
	strncpy(result, reversed, len - prefix_len);
	result[len - prefix_len] = '\0';
	strcat(result, s);
	
	free(reversed);
	free(concat);
	free(lps);

	return result;
}

void runTest(const char* input, const char* expected) {
	char* result = shortestPalindrome(input);
	printf("Input: \"%s\"\n", input);
	printf("Expected Output: \"%s\"\n", expected);
	printf("Actual Output: \"%s\"\n", result);
	assert(strcmp(result, expected) == 0);
	printf("Test Passed!\n\n");
	free(result);
}

int main() {
	runTest("aacecaaa", "aaacecaaa");

	runTest("abcd", "dcbabcd");

	runTest("", "");

	runTest("a", "a");

	runTest("aba", "aba");

	runTest("abababababababababababa", "ababababababababababababa");

	printf("All tests passed successfully!\n");
	return 0;
}


