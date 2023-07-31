namespace csharp
{
    public class Solution
    {
        public int SingleNumber(int[] nums)
        {
            int answer = 0;

            foreach (int i in nums)
            {
                answer ^= i;
            }

            return answer;
        }
    }
}
