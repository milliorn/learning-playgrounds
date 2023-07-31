#include <cstdio>
#include <vector>

class Solution {
public:
  std::vector<double> convertTemperature(double celsius) {
    std::vector<double> result = { celsius + 273.15, celsius * 1.80 + 32.00 };
    return result;
  }
};

int main()
{
    printf("hello from %s!\n", "leetcode");
    return 0;
}