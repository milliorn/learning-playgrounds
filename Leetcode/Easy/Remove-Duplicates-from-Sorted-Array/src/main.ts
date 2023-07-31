function removeDuplicates(nums: number[]): number {
  if (nums.length == 0)
    return 0;

  let tempA = 0;
  let tempB = 0;
  let result = 1;

  while (tempB < nums.length) {
    if (nums[ tempA ] == nums[ tempB ]) {
      tempB++;
    } else {
      nums[ ++tempA ] = nums[ tempB++ ];
      result++;
    }
  }
  return result;
};