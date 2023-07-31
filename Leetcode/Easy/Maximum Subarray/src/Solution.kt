internal class Solution {
    fun maxSubArray(nums: IntArray): Int {
        var max = nums[0]
        var min = 0
        var sum = 0
        for (num in nums) {
            sum += num
            max = Math.max(max, sum - min)
            min = Math.min(min, sum)
        }
        return max
    }
}