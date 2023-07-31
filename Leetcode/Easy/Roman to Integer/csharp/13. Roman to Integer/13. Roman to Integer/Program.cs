namespace Leetcode
{
    public class Solution
    {
        public int RomanToInt(string s) => SetResult(s, s.Length, numerals[s[s.Length - 1]]);

        private static int SetResult(string s, int length, int result)
        {
            for (int i = length - 2; i >= 0; i--)
            {
                result = GetRomanInt(s, result, i, s[i]);
            }
            return result;
        }

        private static int GetRomanInt(string s, int result, int i, char temp) => numerals[temp] >= numerals[s[i + 1]] ? result += numerals[temp] : result -= numerals[temp];

        private static readonly Dictionary<char, int> numerals = new()
        {
            { 'I', 1 },
            { 'V', 5 },
            { 'X', 10 },
            { 'L', 50 },
            { 'C', 100 },
            { 'D', 500 },
            { 'M', 1000 }
        };
    }
}