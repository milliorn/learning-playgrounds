class Solutions {
  def merge(nums1: Array[Int], m: Int, nums2: Array[Int], n: Int) = {
    var a = m + n - 1
    var b = m - 1
    var c = n - 1
    while ( {
      c >= 0
    }) if (b >= 0) nums1({
      a -= 1; a + 1
    }) = if (nums1(b) > nums2(c)) nums1({
      b -= 1; b + 1
    })
    else nums2({
      c -= 1; c + 1
    })
    else nums1({
      a -= 1; a + 1
    }) = nums2({
      c -= 1; c + 1
    })
  }
}