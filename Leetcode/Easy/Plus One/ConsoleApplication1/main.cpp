#include <cstdio>
#include <vector>

class Solution {
public:
  std::vector<int> plusOne(std::vector<int>& digits) {
    std::vector<int> digitsCopy(digits);

    for (int i = digitsCopy.size() - 1; i >= 0; i--)
    {
      if (digitsCopy[i] != 9)
      {
        digitsCopy[i]++;
        break;
      }
      else
      {
        digitsCopy[i] = 0;
      }
    }

    if (digitsCopy[0] == 0)
    {
      std::vector<int> temp(digitsCopy.size() + 1, 0);
      temp[0] = 1;
      return temp;
    }

    return digitsCopy;
  }
};

int main()
{
    printf("hello from %s!\n", "ConsoleApplication1");
    return 0;
}