#include <cstdio>

class Solution
{
public:
  bool checkPerfectNumber(int num)
  {
    if (num <= 1)
      return false;

    int sum = 1;
    int divisor = 2;

    while (divisor * divisor <= num)
    {
      if (num % divisor == 0)
      {
        sum += divisor;
        if (divisor != num / divisor)
        {
          sum += num / divisor;
        }
      }
      divisor += 1;
    }
    return sum == num;
  }
};

int main()
{
  printf("hello from %s!\n", "CPP");
  return 0;
}