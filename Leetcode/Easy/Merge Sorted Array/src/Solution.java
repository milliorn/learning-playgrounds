class Solution {
  public void merge(int[] nums1, int m, int[] nums2, int n) {
    int a = m + n - 1;
    int b = m - 1;
    int c = n - 1;

    while (c >= 0) {
      if (b >= 0) {
        nums1[a--] = nums1[b] > nums2[c] ? nums1[b--] : nums2[c--];
      } else {
        nums1[a--] = nums2[c--];
      }
    }
  }
}
