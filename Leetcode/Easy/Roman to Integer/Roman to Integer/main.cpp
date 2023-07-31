#include <iostream>
#include <string>
#include <unordered_map>

using namespace std;

class Solution
{
public:
    int romanToInt(string s)
    {
        unordered_map<char, int> numerals = {{'I', 1},   {'V', 5},   {'X', 10},
            {'L', 50},  {'C', 100}, {'D', 500},
            {'M', 1000}
        };
        int result = numerals[s[s.length() - 1]];

        for (int i = s.length() - 2; i >= 0; i--)
        {
            char temp = s[i];

            if (numerals[temp] >= numerals[s[i + 1]])
            {
                result += numerals[temp];
            }
            else
            {
                result -= numerals[temp];
            }
        }

        return result;
    }
};

int main()
{
    Solution solution;
    int three = solution.romanToInt("III");
    int fiveEight = solution.romanToInt("LVIII");
    int oneNineNineFour = solution.romanToInt("MCMXCIV");

    cout << three << endl;
    cout << fiveEight << endl;
    cout << oneNineNineFour << endl;

    return 0;
}
