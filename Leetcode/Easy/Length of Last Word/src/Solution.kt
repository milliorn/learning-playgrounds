class Solution {
    fun lengthOfLastWord(s: String): Int {
        var s = s
        var result = 0
        s = s.trim { it <= ' ' }
        for (i in s.length - 1 downTo 0) {
            if (s[i] == ' ') {
                return result
            } else {
                result++
            }
        }
        return 0
    }
}