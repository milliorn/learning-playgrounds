/**
 * @param {number} x
 * @return {number}
 */
var mySqrt = function (x) {
  let result = x;
  while ((result - x / result) > 0.00001) {
    result = (result + x / result) / 2;
  }
  return result | 0;
};