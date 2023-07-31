class mainKT {
    fun getConcatenation(nums: IntArray): IntArray {
        val numscopy = nums.clone()
        val both = nums.copyOf(nums.size + numscopy.size)
        System.arraycopy(numscopy, 0, both, nums.size, numscopy.size)
        return both
    }

    companion object {
        @JvmStatic
        fun main(args: Array<String>) {
            println("Hello world!")
        }
    }
}