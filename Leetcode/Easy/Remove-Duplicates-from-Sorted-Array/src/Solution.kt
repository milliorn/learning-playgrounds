class Solution {
    fun removeDuplicates(nums: IntArray?): Int {
        return if (nums == null || nums.size == 0) 0 else calculate(nums, nums.size, 0, 0, 1)
    }

    private fun calculate(nums: IntArray, len: Int, tempA: Int, tempB: Int, result: Int): Int {
        var tempA = tempA
        var tempB = tempB
        var result = result
        while (tempB < len) {
            if (nums[tempA] == nums[tempB]) {
                tempB++
            } else {
                nums[++tempA] = nums[tempB++]
                result++
            }
        }
        return result
    }
}