internal class Solution {
    fun intToRoman(num: Int): String {
        var num = num
        val answer = StringBuilder()
        val romanInts = intArrayOf(1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1)
        val romanNumerals = arrayOf(
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"
        )
        for (i in romanInts.indices) {
            while (num >= romanInts[i]) {
                num -= romanInts[i]
                answer.append(romanNumerals[i])
            }
        }
        return answer.toString()
    }
}