/**
 * @param {number[]} nums
 * @param {number} val
 * @return {number}
 */
const removeElement = (nums, val) => {
  let count = 0;
  let i;
  for (i = 0; i < nums.length; i++) {
    if (nums[ i ] != val) {
      nums[ count++ ] = nums[ i ];
    }
  }
  return count;
};
