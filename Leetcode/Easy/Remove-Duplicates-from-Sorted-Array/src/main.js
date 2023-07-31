/**
 * @param {number[]} nums
 * @return {number}
 */
const removeDuplicates = (nums) => {
  if (nums.length == 0)
    return 0;

  let tempA = 0;
  let tempB = 0;
  let result = 1;

  iterate();
  return result;

  function iterate() {
    while (tempB < nums.length) {
      calculate();
    }
  }

  function calculate() {
    if (nums[ tempA ] == nums[ tempB ]) {
      tempB++;
    } else {
      nums[ ++tempA ] = nums[ tempB++ ];
      result++;
    }
  }
};