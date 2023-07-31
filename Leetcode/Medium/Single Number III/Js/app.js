'use strict';

/**
 * @param {number[]} nums
 * @return {number[]}
 */
const singleNumber = (nums) => {
    /* Compute the XOR of all elements in the array */
    let xor = 0;

    for (let i = 0; i < nums.length; i++) {
        xor ^= nums[i];
    }

    /* Find the rightmost set bit of the XOR result */
    const rightmost_set_bit = xor & -xor;

    /* Partition the numbers into two groups based on the rightmost set bit */
    const answer = [0, 0];

    for (let i = 0; i < nums.length; i++) {
        const num = nums[i];

        if ((num & rightmost_set_bit)) {
            answer[0] ^= num;
        } else {
            answer[1] ^= num;
        }
    }

    return answer;
};