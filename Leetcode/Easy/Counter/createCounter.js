/**
 * @param {number} n
 * @return {Function} counter
 */
var createCounter = function (n) {
  return function () {
    var result = n;
    n++;
    return result;
  };
};
