#include <string>
class Solution {
public:
  std::string defangIPaddr(std::string address) {
    std::string result = "";

    for (char ch : address)
    {
      if (ch == '.') result += "[.]";
      else result += ch;
    }

    return result;
  }
};

#include <iostream>

int main()
{
  std::cout << "Hello World!\n";
}
