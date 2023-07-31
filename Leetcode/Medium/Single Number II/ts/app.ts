function singleNumber(nums: number[]): number {
  let unique = 0;
  let duplicate = 0;

  nums.forEach((e) => {
    // Calculate the new value for "unique" using the XOR and NOT bitwise operators. This operation ensures that only the bits that are unique to the current element are included in the new "unique" value.
    unique = (unique ^ e) & ~duplicate;
    // Calculate the new value for "duplicate" using the XOR and NOT bitwise operators. This operation ensures that only the bits that are duplicated in the current element are included in the new "duplicate" value.
    duplicate = (duplicate ^ e) & ~unique;
  });

  return unique;
}
