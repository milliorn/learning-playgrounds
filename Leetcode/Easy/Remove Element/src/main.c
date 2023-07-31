int removeElement(int *nums, int numsSize, int val) {
  if (numsSize == 0)
    return 0;

  int count = 0;
  int i;

  for (i = 0; i < numsSize; i++) {
    if (nums[i] != val) {
      nums[count++] = nums[i];
    }
  }
  return count;
}