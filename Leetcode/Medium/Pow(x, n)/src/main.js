/**
 * @param {number} x
 * @param {number} n
 * @return {number}
 */
var myPow = function (x, n) {
  if (n === 0) return 1;
  if (n > 0) return pow(x, n);
  if (n < 0) return 1 / pow(x, -n);
};

const pow = function (x, n) {
  if (n === 1) return x;
  const num = pow(x, Math.floor(n / 2));
  return n % 2 === 0 ? num * num : x * num * num;
};