namespace RremoveElement
{
    public class Solution
    {
        public int RemoveElement(int[] nums, int val) => nums == null || nums.Length == 0 ? 0 : Calculate(nums, val);

        private static int Calculate(int[] nums, int val)
        {
            int count = 0;

            for (int i = 0; i < nums.Length; i++)
            {
                if (nums[i] != val)
                {
                    nums[count++] = nums[i];
                }
            }

            return count;
        }
    }
}