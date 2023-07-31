function mySqrt(x: number): number {
  let result = x;
  while ((result - x / result) > 0.00001) {
    result = (result + x / result) / 2;
  }
  return result | 0;
};