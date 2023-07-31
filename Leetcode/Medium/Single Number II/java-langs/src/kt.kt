class kt {
    fun singleNumber(nums: IntArray): Int {
        var unique = 0
        var duplicate = 0
        for (num in nums) {
            // Calculate the new value for "unique" using the XOR and NOT bitwise operators.
            // This operation ensures that only the bits that are unique to the current element are included in the new "unique" value.
            unique = unique xor num and duplicate.inv()
            // Calculate the new value for "duplicate" using the XOR and NOT bitwise operators.
            // This operation ensures that only the bits that are duplicated in the current element are included in the new "duplicate" value.
            duplicate = duplicate xor num and unique.inv()
        }
        return unique
    }

    companion object {
        @JvmStatic
        fun main(args: Array<String>) {
            println("Hello world!")
        }
    }
}