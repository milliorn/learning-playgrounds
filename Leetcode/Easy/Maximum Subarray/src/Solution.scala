class Solution {
  def maxSubArray(nums: Array[Int]) = {
    var max = nums(0)
    var min = 0
    var sum = 0
    for (num <- nums) {
      sum += num
      max = Math.max(max, sum - min)
      min = Math.min(min, sum)
    }
    max
  }
}