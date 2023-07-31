class kt {
    fun singleNumber(nums: IntArray): Int {
        var answer = 0
        for (i in nums) {
            answer = answer xor i
        }
        return answer
    }
}