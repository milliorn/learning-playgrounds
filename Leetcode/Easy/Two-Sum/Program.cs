namespace TwoSum
{
  public class Solution
  {
    public int[] TwoSum(int[] nums, int target)
    {
      for (int x = 0; x < nums.GetLength(0); x++)
      {
        for (int y = x + 1; y < nums.GetLength(0); y++)
        {
          if ((nums[x] + nums[y]) == target)
          {
            return new[] { x, y };
          }
        }
      }
      return nums;
    }
  }
}
