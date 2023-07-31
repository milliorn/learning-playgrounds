#include <cstdio>
#include <vector>
#include <cstdint>
#include <limits>

int thirdMax(std::vector<int>& nums) {
  const int64_t emptyMax = std::numeric_limits<int64_t>::min();
  int64_t max1 = emptyMax;
  int64_t max2 = emptyMax;
  int64_t max3 = emptyMax;

  for (const auto& element : nums) {
    if (element == max1 || element == max2 || element == max3) {
      continue;
    }

    if (element > max1) {
      max3 = max2;
      max2 = max1;
      max1 = element;
    }
    else if (element > max2) {
      max3 = max2;
      max2 = element;
    }
    else if (element > max3) {
      max3 = element;
    }
  }

  return max3 == emptyMax ? static_cast<int>(max1) : static_cast<int>(max3);
}

int main() {
  printf("hello from %s!\n", "CPP");
  return 0;
}
