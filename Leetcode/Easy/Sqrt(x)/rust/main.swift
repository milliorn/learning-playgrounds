class Solution {
    func mySqrt(_ x: Int) -> Int {
      if (x == 0 || x == 1) {
        return x;
      }

      var left = 1;
      var right = x;
      var res = 1;

      while (left <= right) {
        var mid = (right - left) / 2 + left;
          if (x / mid == mid) {
            return mid;
          }
          if (mid > x / mid) {
            right = mid - 1;
          } else {
            left = mid + 1;
            res = mid;
          }
      }
        return res;
    }
}