using System.Text.RegularExpressions;

namespace LeetCode
{
    public class Solution
    {
        public int LengthOfLastWord(string s) => Regex.Replace(s.Trim(), @"\s+", " ").Split(" ").Last().Length;
    }
}