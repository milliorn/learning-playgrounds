class Solution {
    fun plusOne(digits: IntArray): IntArray {
        val digitsClone = digits.clone()
        for (i in digitsClone.indices.reversed()) {
            if (digitsClone[i] != 9) {
                digitsClone[i]++
                break
            } else {
                digitsClone[i] = 0
            }
        }
        if (digitsClone[0] == 0) {
            val temp = IntArray(digitsClone.size + 1)
            temp[0] = 1
            return temp
        }
        return digitsClone
    }
}