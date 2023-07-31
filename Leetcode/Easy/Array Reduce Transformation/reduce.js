/**
 * @param {number[]} nums
 * @param {Function} fn
 * @param {number} init
 * @return {number}
 */
function reduce(nums, fn, init) {
  let amswer = init;
  for (let i = 0; i < nums.length; i++) {
    amswer = fn(amswer, nums[i]);
  }
  return amswer;
}
