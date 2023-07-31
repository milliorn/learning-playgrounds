/**
 Do not return anything, modify nums1 in-place instead.
 */
function merge(nums1: number[], m: number, nums2: number[], n: number): void {
  let a = m + n - 1;
  let a1 = m - 1;
  let a2 = n - 1;

  while (a2 >= 0) {
    if (a1 >= 0) {
      nums1[a--] = nums1[a1] > nums2[a2] ? nums1[a1--] : nums2[a2--];
    } else {
      nums1[a--] = nums2[a2--];
    }
  }
};