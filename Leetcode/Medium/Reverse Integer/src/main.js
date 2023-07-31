/**
 * @param {number} x
 * @return {number}
 */
var reverse = function (x) {
  const swap = parseInt(Math.abs(x).toString().split('').reverse().join(''));
  return swap > 2147483647 ? 0 : x < 0 ? -Math.abs(swap) : swap;
};