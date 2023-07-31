using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace leetcode
{
    public class Solution
    {
        public double[] ConvertTemperature(double celsius)
        {
            return new double[] { celsius + 273.15, celsius * 1.80 + 32.00 };
        }
    }

    internal class Program
    {
        static void Main(string[] args)
        {
            Solution solution = new Solution();

            foreach (var item in solution.ConvertTemperature(36.50))
            {
                Console.WriteLine(item.ToString());
            }
        }
    }
}
