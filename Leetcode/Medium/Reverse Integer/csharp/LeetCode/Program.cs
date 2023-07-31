namespace LeetCode
{
    public class Solution
    {
        public int Reverse(int x)
        {
            long result = 0;

            while (x != 0)
            {
                result = result * 10 + x % 10;
                if (result is > Int32.MaxValue or < Int32.MinValue)
                {
                    return 0;
                }
                x /= 10;
            }
            return (int)result;
        }

        static void Main()
        {
            Solution solution = new();
            Console.WriteLine(solution.Reverse(123));
            Console.WriteLine(solution.Reverse(-123));
            Console.WriteLine(solution.Reverse(120));
        }
    }
}