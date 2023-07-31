namespace LeetCode
{
    public class Solution
    {
        public int MySqrt(int x)
        {
            /* Babylonian method */
            double precision = 0.00001;
            double s = x;

            while ((s - x / s) > precision)
            {
                s = (s + x / s) / 2;
            }
            return (int)s;
        }

        static void Main()
        {
            Solution solution = new Solution();
            Console.WriteLine(solution.MySqrt(4));
            Console.WriteLine(solution.MySqrt(8));
            Console.WriteLine(solution.MySqrt(2147395599));
        }
    }
}
