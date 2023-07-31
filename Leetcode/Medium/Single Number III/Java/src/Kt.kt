class Kt {
    fun singleNumber(nums: IntArray): IntArray {
        /* Compute the XOR of all elements in the array */
        var xor = 0
        for (i in nums.indices) {
            xor = xor xor nums[i]
        }

        /* Find the rightmost set bit of the XOR result */
        val rightmost_set_bit = xor and -xor

        /* Partition the numbers into two groups based on the rightmost set bit */
        val answer = intArrayOf(0, 0)
        for (i in nums.indices) {
            val num = nums[i]
            if (num and rightmost_set_bit == 0) {
                answer[0] = answer[0] xor num
            } else {
                answer[1] = answer[1] xor num
            }
        }
        return answer
    }

    companion object {
        @JvmStatic
        fun main(args: Array<String>) {
            println("Hello world!")
        }
    }
}