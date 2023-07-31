internal class Solution {
    fun reverse(x: Int): Int {
        var x = x
        var result: Long = 0
        while (x != 0) {
            result = result * 10 + x % 10
            if (result > Int.MAX_VALUE || result < Int.MIN_VALUE) {
                return 0
            }
            x /= 10
        }
        return result.toInt()
    }
}