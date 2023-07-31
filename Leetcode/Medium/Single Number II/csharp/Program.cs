using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace csharp
{
    public class Solution
    {
        // The "SingleNumber" method uses a bitwise operation to find the unique number in the array, meaning the number that appears only once in the array while all others appear twice. The bitwise operation used here is XOR (^) and NOT (~).
        public int SingleNumber(int[] nums)
        {
            int unique = 0;
            int duplicate = 0;

            foreach (int num in nums)
            {
                // sets the "unique" variable to the result of XORing the current "unique" value with the current "num" value and then ANDing the result with the NOT of the "duplicate" value. This operation ensures that only the bits that are unique to the current "num" value are included in the new "unique" value.
                unique = (unique ^ num) & ~duplicate;
                // sets the "duplicate" variable to the result of XORing the current "duplicate" value with the current "num" value and then ANDing the result with the NOT of the "unique" value. This operation ensures that only the bits that are duplicated in the current "num" value are included in the new "duplicate" value.
                duplicate = (duplicate ^ num) & ~unique;
            }

            return unique;
        }
    }

    internal class Program
    {
        static void Main(string[] args)
        {
        }
    }
}
