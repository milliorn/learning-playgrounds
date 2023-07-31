#include <cstdio>
#include <vector>

class Solution {
public:
  std::vector<int> singleNumber(std::vector<int>& nums) {
    /* Compute the XOR of all elements in the array */
    int exclusiveOr = 0;

    for (int num : nums) {
      exclusiveOr ^= num;
    }

    /* Find the rightmost set bit of the XOR result */
    int rightmost_set_bit = exclusiveOr & -static_cast<unsigned int>(exclusiveOr);

    /* Partition the numbers into two groups based on the rightmost set bit */
    std::vector<int> answer = { 0, 0 };

    for (int num : nums) {
      if ((num & rightmost_set_bit) == 0) {
        answer[0] ^= num;
      }
      else {
        answer[1] ^= num;
      }
    }

    return answer;
  }
};

int main()
{
  printf("hello from %s!\n", "Cpp");
  return 0;
}