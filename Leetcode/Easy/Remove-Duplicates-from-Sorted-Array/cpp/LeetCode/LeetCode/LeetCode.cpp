#include <iostream>
#include <string>
#include <vector>


class Solution {
public:
  int removeDuplicates(std::vector<int>& nums) {

    return nums.empty() || nums.size() == 0 ? 0 : Calculate(nums);
  }

  int Calculate(std::vector<int>& nums)
  {
    int tempA = 0;
    int tempB = 0;
    int result = 1;

    while (tempB < nums.size()) {
      if (nums[tempA] == nums[tempB]) {
        tempB++;
      }
      else {
        nums[++tempA] = nums[tempB++];
        result++;
      }
    }

    return result;
  }
};

int main() {
  Solution solution;
  std::vector<int> vec = { 1, 1, 2 };
  int temp = solution.removeDuplicates(vec);
  std::cout << temp << std::endl;
}
