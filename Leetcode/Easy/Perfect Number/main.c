#include <stdbool.h>

bool checkPerfectNumber(int num)
{
  // Check if num is less than or equal to 1
  if (num <= 1)
    return false;

  // Initialize sum to 1 (the first proper divisor)
  int sum = 1;
  int divisor = 2;

  // Iterate through all possible divisors of num
  while (divisor * divisor <= num)
  {
    // Check if i is a divisor of num
    if (num % divisor == 0)
    {
      // Add i to sum
      sum += divisor;
      // If i is not the square root of num, add num/i to sum
      if (divisor != num / divisor)
      {
        sum += num / divisor;
      }
    }
    divisor += 1;
  }
  // Check if sum is equal to num (num is a perfect number if so)
  return sum == num;
}