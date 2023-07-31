namespace LeetCode
{
    public class Solution
    {
        public static int RemoveDuplicates(int[] nums)
        {
            if (nums == null || nums.Length == 0) return 0;

            int tempA = 0;
            int tempB = 0;

            return Calculate(nums, ref tempA, ref tempB, 1);


            static int Calculate(int[] nums, ref int tempA, ref int tempB, int result)
            {
                while (tempB < nums.Length)
                {
                    if (nums[tempA] == nums[tempB])
                    {
                        tempB++;
                    }
                    else
                    {
                        nums[++tempA] = nums[tempB++];
                        result++;
                    }
                }
                return result;
            }
        }

        static void Main(string[] args)
        {
            int[] nums = { 1, 1, 2 };
            Solution solution = new();
            int result = RemoveDuplicates(nums);
            Console.WriteLine(result);
        }
    }
}