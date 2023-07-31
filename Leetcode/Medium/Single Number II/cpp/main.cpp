#include <cstdio>
#include <vector>
#include <algorithm>

class Solution {
public:
  int singleNumber(std::vector<int>& nums) {
    std::sort(nums.begin(), nums.end());
    int i;
    for (i = 0; i < nums.size(); i += 3)
    {
      if (i == nums.size() - 1 || nums[i] != nums[i + 1])
        break;
    }
    return nums[i];
  }
};

int main()
{
    printf("hello from %s!\n", "cpp");
    return 0;
}