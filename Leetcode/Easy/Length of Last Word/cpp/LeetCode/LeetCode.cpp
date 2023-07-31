#include <iostream>
#include <string>
#include <sstream>

class Solution {
public:
  int lengthOfLastWord(std::string s) {
    std::stringstream ss(s);
    std::string str;
    int i = 0;
    while (ss >> str) {
      i = str.size();
    }
    return i;
  }
};

int main()
{
  Solution solution;
  int a = solution.lengthOfLastWord("luffy is still joyboy");
  std::cout << a << std::endl;
  return 0;
}
