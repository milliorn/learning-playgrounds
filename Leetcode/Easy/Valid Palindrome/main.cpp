#include <algorithm>
#include <cctype>
#include <string>

class Solution
{
public:
  bool isPalindrome(std::string s)
  {
    // copy string so we don't mutate the original
    std::string formattedStr = s;

    // convert string to all lowercase
    std::transform(formattedStr.begin(), formattedStr.end(), formattedStr.begin(), [](unsigned char c)
                   { return std::tolower(c); });

    // remove all non-alphanumeric characters from the string
    formattedStr.erase(std::remove_if(formattedStr.begin(), formattedStr.end(), [](unsigned char c)
                                      { return !std::isalnum(c); }),
                       formattedStr.end());

    // copy formatted string and reverse it
    std::string reversedStr = formattedStr;

    // reverse the string
    std::reverse(formattedStr.begin(), formattedStr.end());

    // check if we are palindrome
    return formattedStr == reversedStr;
  }
};
