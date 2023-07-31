#include <iostream>

class Solution {
public:
  int mySqrt(int x) {
    double precision = 0.00001;
    double result = x;
    while ((result - x / result) > precision) {
      result = (result + x / result) / 2;
    }
    return (int)result;
  }
};

int main()
{
  std::cout << "Hello World!\n";
}
