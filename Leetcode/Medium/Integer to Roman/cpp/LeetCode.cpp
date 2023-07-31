#include <iostream>
#include <string>
#include <vector>

class Solution {
public:
  std::string intToRoman(int num) {
    std::string answer = "";
    std::vector<int>romanInts = { 1000,900,500,400,100,90,50,40,10,9,5,4,1 };
    std::vector<std::string> romanNumerals{ "M","CM", "D","CD","C","XC", "L","XL","X","IX","V","IV","I" };
    return convertRoman(romanInts, num, answer, romanNumerals);
  }

  std::string convertRoman(std::vector<int>& romanInts, int& num, std::string& answer, std::vector<std::string>& romanNumerals)
  {
    for (int i = 0; i < romanInts.size(); ++i) {
      while (num >= romanInts[i]) {
        num -= romanInts[i];
        answer += romanNumerals[i];
      }
    }
    return answer;
  }
};

int main()
{
  std::cout << "Hello World!\n";
}
