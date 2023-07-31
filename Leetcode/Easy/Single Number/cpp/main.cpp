#include <cstdio>
#include <vector>

class Solution {
public:
  int singleNumber(std::vector<int>& nums) {
    int answer = 0;

    for (int i : nums) {
      answer ^= i;
    }

    return answer;
  }
};

int main()
{
  printf("hello from %s!\n", "cpp");
  return 0;
}