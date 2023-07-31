'use strict';

/**
 * @param {number[]} nums
 * @return {number}
 */
const singleNumber = nums => {
    let answer = 0;
    nums.forEach(e => answer = answer ^ e);
    return answer;
};

console.log(singleNumber([2, 2, 1]));
console.log(singleNumber([4, 1, 2, 1, 2]));
console.log(singleNumber([1]));
