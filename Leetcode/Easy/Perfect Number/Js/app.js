"use strict";

/**
 * @param {number} num
 * @return {boolean}
 */
function checkPerfectNumber(num) {
  if (num <= 1)
    return false;

  let sum = 1;
  let divisor = 2;

  while (divisor * divisor <= num) {
    if (num % divisor == 0) {
      sum += divisor;
      if (divisor != num / divisor) {
        sum += num / divisor;
      }
    }
    divisor += 1;
  }
  return sum == num;
}
