#include <cstdio>
#include <vector>

class Solution {
public:
  std::vector<int> buildArray(std::vector<int>& nums) {
    std::vector<int> ans;

    for (size_t i = 0; i < nums.size(); i++)
    {
      ans.push_back(nums[nums[i]]);
    }

    return ans;
  }

  int main()
  {
    printf("hello from %s!\n", "CPP");
    return 0;
  }
};