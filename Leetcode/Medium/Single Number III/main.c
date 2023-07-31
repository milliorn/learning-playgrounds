#include <stdlib.h>

int *singleNumber(int *nums, int numsSize, int *returnSize)
{
  /* Compute the XOR of all elements in the array */
  int exclusiveOr = 0;

  for (int i = 0; i < numsSize; i++)
  {
    exclusiveOr ^= nums[i];
  }

  /* Find the rightmost set bit of the XOR result */
  int rightmost_set_bit = exclusiveOr & -((unsigned int)exclusiveOr);

  /* Partition the numbers into two groups based on the rightmost set bit */
  /* Necessary to allocate dynamic memory for the result array answer.*/
  int *answer = (int *)malloc(2 * sizeof(int));
  answer[0] = 0;
  answer[1] = 0;

  for (int i = 0; i < numsSize; i++)
  {
    if ((nums[i] & rightmost_set_bit) == 0)
    {
      answer[0] ^= nums[i];
    }
    else
    {
      answer[1] ^= nums[i];
    }
  }

  /* Since we cannot return an array directly in C, we need to allocate dynamic memory for the result array using malloc and return a pointer to it. */
  *returnSize = 2;
  return answer;
}