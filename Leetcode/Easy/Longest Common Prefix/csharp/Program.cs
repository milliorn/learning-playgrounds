using System;

namespace csharp
{
    public class Solution
    {
        public string LongestCommonPrefix(string[] strs)
        {
            string prefix = strs[0];
            int position = 0;
            int maxCharIndex = strs[0].Length;

            for (int i = 1; i < strs.Length; ++i)
            {
                maxCharIndex = Math.Min(maxCharIndex, strs[i].Length);
            }

            while (position < maxCharIndex)
            {
                char previousLetter = strs[0][position];

                for (int i = 1; i < strs.Length; ++i)
                {
                    if (previousLetter == strs[i][position])
                    {
                        continue;
                    }
                    return prefix.Substring(0, position);
                }
                ++position;
                prefix += previousLetter;
            }
            return prefix.Substring(0, position);
        }

        public static void Main()
        {
            string[] test1 = new string[] { "dog", "racecar", "car" };
            string[] test2 = new string[] { "flower", "flow", "flight" };
            string[] test3 = new string[] { "", "" };

            Solution solution = new Solution();

            Console.WriteLine(solution.LongestCommonPrefix(test1));
        }
    }
}
