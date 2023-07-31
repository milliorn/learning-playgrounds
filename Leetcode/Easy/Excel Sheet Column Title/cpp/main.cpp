#include <cstdio>
#include <string>

class Solution {
public:
  std::string convertToTitle(int columnNumber) {
    std::string result = "";

    // Keep iterating until columnNumber becomes zero
    while (columnNumber > 0) {
      // Get the remainder after dividing columnNumber by 26
      int remainder = (columnNumber - 1) % 26;

      // Convert the remainder to a character and add it to the result
      result = char(remainder + 'A') + result;

      // Update columnNumber by subtracting the remainder and dividing by 26
      columnNumber = (columnNumber - 1) / 26;
    }

    // Return the resulting string
    return result;
  }
};


int main()
{
  printf("hello from %s!\n", "cpp");
  return 0;
}