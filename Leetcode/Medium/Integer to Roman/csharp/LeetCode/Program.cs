using System.Text;

namespace LeetCode
{
    public class Solution
    {
        public string IntToRoman(int num)
        {
            StringBuilder answer = new();
            int[] romanInts = { 1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1 };
            string[] romanNumerals = { "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I" };

            for (int i = 0; i < romanInts.Length; i++)
            {
                while (num >= romanInts[i])
                {
                    num -= romanInts[i];
                    answer.Append(romanNumerals[i]);
                }
            }
            return answer.ToString();
        }
    }
}