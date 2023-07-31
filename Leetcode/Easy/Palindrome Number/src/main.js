/**
 * @param {number} x
 * @return {boolean}
 */
const isPalindrome = x => x >= 0 ? x.toString() === x.toString().split("").reverse().join("") : false;