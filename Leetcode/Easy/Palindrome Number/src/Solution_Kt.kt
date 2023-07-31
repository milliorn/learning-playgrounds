import java.lang.StringBuilder

class Solution {
    fun isPalindrome(x: Int): Boolean {
        if (x < 0) return false
        val temp = x.toString()
        return temp == StringBuilder(temp).reverse().toString()
    }
}