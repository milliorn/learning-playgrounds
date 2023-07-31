class Solution {
  public int maxSubArray(int[] nums) {
    int max = nums[0], min = 0, sum = 0;
    for (int num : nums) {
      sum += num;
      max = Math.max(max, sum - min);
      min = Math.min(min, sum);
    }
    return max;
  }
}
