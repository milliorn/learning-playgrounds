int mySqrt(int x) {
  double result = x;
  while ((result - x / result) > 0.00001) {
    result = (result + x / result) / 2;
  }
  return (int)result;
}