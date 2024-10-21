
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

#define MAX_STICKERS 50
#define MAX_STICKER_LEN 15
#define MAX_TARGET_LEN 15
#define ALPHABET_SIZE 26

int min(int a, int b) {
	return (a < b) ? a : b;
}

int minStickers(char** stickers, int stickerSize, char* target) {
	int targetLen = strlen(target);
	int dp[1 << 15];
	memset(dp, -1, sizeof(dp));
	dp[0] = 0;

	for (int state = 0; state < (1 << targetLen); state++) {
		if (dp[state] == -1) continue;

		for (int i = 0; i < stickerSize; i++) {
			int newState = state;
			int stickerCount[26] = {0};
			for (int j = 0; stickers[i][j]; j++) {
				stickerCount[stickers[i][j] - 'a']++;
			}

			for (int j = 0; j < targetLen; j++) {
				if (((newState >> j) & 1) == 0 && stickerCount[target[j] - 'a'] > 0) {
					newState |= 1 << j;
					stickerCount[target[j] - 'a']--;
				}
			}

			if (newState != state) {
				if (dp[newState] == -1) dp[newState] = dp[state] + 1;
				else dp[newState] = min(dp[newState], dp[state] + 1);
			}
		}
	}

	return dp[(1 << targetLen) - 1];
}

int main() {
	char* stickers[] = {"with", "example", "science"};
	char target[] = "thehat";
	int stickerSize = 3;

	int result = minStickers(stickers, stickerSize, target);
	printf("Minimum number of stickers needed: %d\n", result);

	return 0;
}


