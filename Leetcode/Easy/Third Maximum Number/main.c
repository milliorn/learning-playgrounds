#include <limits.h>
#include <cstdint>

int thirdMax(int *nums, int numsSize)
{
  const int64_t kEmptyMax = INT64_MIN;
  int64_t max1 = kEmptyMax;
  int64_t max2 = kEmptyMax;
  int64_t max3 = kEmptyMax;

  for (int i = 0; i < numsSize; i++)
  {
    int64_t element = nums[i];

    if (element == max1 || element == max2 || element == max3)
    {
      continue;
    }

    if (element > max1)
    {
      max3 = max2;
      max2 = max1;
      max1 = element;
    }
    else if (element > max2)
    {
      max3 = max2;
      max2 = element;
    }
    else if (element > max3)
    {
      max3 = element;
    }
  }

  return max3 != kEmptyMax ? (int)max3 : (int)max1;
}
