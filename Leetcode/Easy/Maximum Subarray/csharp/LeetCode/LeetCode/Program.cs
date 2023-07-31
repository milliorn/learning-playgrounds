namespace LeetCode
{
    public class Solution
    {
        public int MaxSubArray(int[] nums)
        {
            int max = nums[0], min = 0, sum = 0;
            foreach (int num in nums)
            {
                sum += num;
                max = Math.Max(max, sum - min);
                min = Math.Min(min, sum);
            }
            return max;
        }

        static void Main(string[] args)
        {

        }
    }
}