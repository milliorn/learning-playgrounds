int removeDuplicates(int *nums, int numsSize) {
  if (numsSize == 0)
    return 0;

  int tempA = 0;
  int tempB = 0;
  int result = 1;

  while (tempB < numsSize) {
    if (nums[tempA] == nums[tempB]) {
      tempB++;
    } else {
      nums[++tempA] = nums[tempB++];
      result++;
    }
  }
  return result;
}