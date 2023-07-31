function maxSubArray(nums: number[]): number {
  let max = nums[ 0 ], min = 0, sum = 0;
  for (const num of nums) {
    sum += num;
    max = Math.max(max, sum - min);
    min = Math.min(min, sum);
  }
  return max;
};