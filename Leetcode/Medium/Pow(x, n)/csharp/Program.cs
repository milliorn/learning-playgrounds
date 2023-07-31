namespace LeetCode
{
    public class Solution
    {
        public double MyPow(double x, int n)
        {
            if (n == Int32.MaxValue) return x;
            else if (n == Int32.MinValue) return (x == 1 || x == -1) ? 1 : 0;

            if (n < 0)
            {
                x = 1 / x;
                n *= -1;
            }

            double answer = 1;
            for (int i = 0; i < n; i++) answer = answer * x;
            return answer;
        }
    }
}