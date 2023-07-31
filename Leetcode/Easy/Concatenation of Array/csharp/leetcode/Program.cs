using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace leetcode
{
    public class Solution
    {
        public int[] GetConcatenation(int[] nums)
        {
            return ((int[])nums.Clone()).Concat(nums).ToArray();
        }
    }

    internal class Program
    {
        static void Main(string[] args)
        {
            Solution solution = new Solution();
            int[] test = new int[] { 1, 2, 1 };
            Console.WriteLine("[{0}]", string.Join(", ", solution.GetConcatenation(test)));
        }
    }
}
