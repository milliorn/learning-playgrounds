/**
 * @param {number[]} nums
 * @return {number}
 */
var maxSubArray = function (nums) {
  let max = nums[ 0 ], min = 0, sum = 0;
  for (const num of nums) {
    sum += num;
    max = Math.max(max, sum - min);
    min = Math.min(min, sum);
  }
  return max;
};