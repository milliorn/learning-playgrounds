internal class Solution {
    fun merge(nums1: IntArray, m: Int, nums2: IntArray, n: Int) {
        var a = m + n - 1
        var b = m - 1
        var c = n - 1
        while (c >= 0) {
            if (b >= 0) {
                nums1[a--] = if (nums1[b] > nums2[c]) nums1[b--] else nums2[c--]
            } else {
                nums1[a--] = nums2[c--]
            }
        }
    }
}