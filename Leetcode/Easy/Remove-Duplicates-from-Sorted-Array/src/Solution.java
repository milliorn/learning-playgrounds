public class Solution {
  public int removeDuplicates(int[] nums) {
    return nums == null || nums.length == 0 ? 0 : calculate(nums, nums.length, 0, 0, 1);
  }

  private int calculate(int[] nums, int len, int tempA, int tempB, int result) {
    while (tempB < len) {
      if (nums[tempA] == nums[tempB]) {
        tempB++;
      } else {
        nums[++tempA] = nums[tempB++];
        result++;
      }
    }
    return result;
  }
}
