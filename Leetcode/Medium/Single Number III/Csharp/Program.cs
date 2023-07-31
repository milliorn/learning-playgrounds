public class Solution
{
    public int[] SingleNumber(int[] nums)
    {
        /* Compute the XOR of all elements in the array */
        int xor = 0;

        /* Partition the numbers into two groups based on the rightmost set bit */
        int[] answer = { 0, 0 };

        foreach (int num in nums)
        {
            xor ^= num;
        }

        /* Find the rightmost set bit of the XOR result */
        int rightmost_set_bit = xor & -xor;

        foreach (var num in nums)
        {
            if ((num & rightmost_set_bit) == 0)
            {
                answer[0] ^= num; 
            }
            else
            {
                answer[1] ^= num;
            }
        }

        return answer;
    }
}