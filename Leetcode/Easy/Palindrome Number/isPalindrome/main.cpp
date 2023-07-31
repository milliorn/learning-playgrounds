#include <iostream>
#include <string>

class Solution
{
public:
    bool isPalindrome(int x)
    {
        if (x < 0)
            return false;
        std::string s = std::to_string(x);
        std::string rev = std::string(s.rbegin(), s.rend());
        return s == rev;
    }
};

const char *bool_cast(const bool b)
{
    return b ? "true" : "false";
}

int main()
{
    Solution solution;
    std::cout << bool_cast(solution.isPalindrome(121)) << "\n";
    std::cout << bool_cast(solution.isPalindrome(10)) << "\n";
    return 0;
}
