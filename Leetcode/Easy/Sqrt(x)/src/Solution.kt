class Solution {
    fun mySqrt(x: Int): Int {
        val precision = 0.00001
        var result = x.toDouble()
        while (result - x / result > precision) {
            result = (result + x / result) / 2
        }
        return result.toInt()
    }
}