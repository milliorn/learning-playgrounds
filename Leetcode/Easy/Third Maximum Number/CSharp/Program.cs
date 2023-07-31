public class Solution
{
  public int ThirdMax(int[] nums)
  {
    long max1 = long.MinValue;
    long max2 = long.MinValue;
    long max3 = long.MinValue;

    foreach (int num in nums)
    {
      if (num > max1)
      {
        max3 = max2;
        max2 = max1;
        max1 = num;
      }
      else if (num > max2 && num < max1)
      {
        max3 = max2;
        max2 = num;
      }
      else if (num > max3 && num < max2)
      {
        max3 = num;
      }
    }

    return max3 != long.MinValue ? (int)max3 : (int)max1;
  }
}