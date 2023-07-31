// leetcode.cpp : Defines the entry point for the application.
//

#include "leetcode.h"
#include <vector>

using namespace std;

class Solution {
public:
  vector<int> getConcatenation(vector<int>& nums) {
    std::vector<int> copyNums(nums);
    copyNums.insert(copyNums.end(), nums.begin(), nums.end());
    return copyNums;
  }
};

int main()
{
	cout << "Hello CMake." << endl;
	return 0;
}
