class Solution {
    fun checkPerfectNumber(num: Int): Boolean {
        if (num <= 1) return false

        var divisorSum = 1
        var divisor = 2

        while (divisor * divisor <= num) {
            if (num % divisor == 0) {
                divisorSum += divisor
                if (divisor != num / divisor) divisorSum += num / divisor
            }
            divisor++
        }
        return divisorSum == num
    }
}
