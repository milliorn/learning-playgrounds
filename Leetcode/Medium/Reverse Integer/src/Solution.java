import static java.lang.Integer.MAX_VALUE;
import static java.lang.Integer.MIN_VALUE;

class Solution {
    public int reverse(int x) {
        long result = 0;
        while (x != 0) {
            result = result * 10 + x % 10;
            if (result > 2147483647 || result < -2147483647) {
                return 0;
            }
            x /= 10;
        }
        return (int) result;
    }
}
