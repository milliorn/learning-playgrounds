namespace Leetcode
{
    public class Solution
    {
        public bool IsPalindrome(int x)
        {
            if (x < 0)
                return false;

            String str = x.ToString();
            return str.Equals(new string(str.Reverse().ToArray()), StringComparison.Ordinal);
        }
    }
}