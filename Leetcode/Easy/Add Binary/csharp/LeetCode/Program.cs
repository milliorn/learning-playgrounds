namespace LeetCode
{
    public class Solution
    {
        public static string AddBinary(string a, string b)
        {
            string result = "";
            int sizeA = a.Length;
            int sizeB = b.Length;
            int rem = 0;

            while (sizeA > 0 || sizeB > 0)
            {
                Calculate(a, b, ref result, sizeA, sizeB, ref rem);
                sizeA--;
                sizeB--;
            }

            return rem == 1 ? rem + result : result;
        }

        private static void Calculate(string a, string b, ref string result, int sizeA, int sizeB, ref int rem)
        {
            int sum = (sizeA <= 0 ? 0 : Convert.ToInt32(a[sizeA - 1].ToString())) + (sizeB <= 0 ? 0 : Convert.ToInt32(b[sizeB - 1].ToString())) + rem;

            if (sum > 1)
            {
                rem = 1;
                result = (sum % 2) + result;
            }
            else
            {
                rem = 0;
                result = sum + result;
            }
        }

        static void Main() => Console.WriteLine(AddBinary("11", "1"));
    }
}