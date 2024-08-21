/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* twoSum(int* nums, int numsSize, int target, int* returnSize) {
    int i, j;
    i = 0;
    j = 0;
    
    returnSize = malloc(2 * sizeof(int));

    for (i = 0; i < numsSize; ++i) {
        for (j = 0; j < numsSize; ++j) {
            if (nums[i] + nums[j] == target) {
                returnSize[0] = nums[i];
                returnSize[1] = nums[j];
            }
        }
    }
    
    return returnSize;
}

int main() {
  //Do not remember the challenge, to be implemented soon!
	int nums = {3, 2, 4};

	int target = 6;

	int* returnSize;
	int numsSize = sizeof(nums) / sizeof(nums[0]);

	printf("twoSum(nums, numsSize, target, returnSize"}
  return (0);
}
