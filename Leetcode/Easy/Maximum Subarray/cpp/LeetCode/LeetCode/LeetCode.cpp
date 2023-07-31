
#include <iostream>
#include <vector>

class Solution {
public:
  int maxSubArray(std::vector<int>& nums) {
    int max = nums[0], min = 0, sum = 0;
    for (int num : nums)
    {
      sum += num;
      max = std::max(max, sum - min);
      min = std::min(min, sum);
    }
    return max;
  }
};

int main()
{
  std::cout << "Hello World!\n";
}
