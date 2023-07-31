class Solution(object):
    def singleNumber(self, nums):
        unique = 0
        duplicate = 0

        for num in nums:
            # Calculate the new value for "unique" using the XOR and NOT bitwise operators. This operation ensures that only the bits that are unique to the current element are included in the new "unique" value.
            unique = (unique ^ num) & ~duplicate
            # Calculate the new value for "duplicate" using the XOR and NOT bitwise operators. This operation ensures that only the bits that are duplicated in the current element are included in the new "duplicate" value.
            duplicate = (duplicate ^ num) & ~unique

        return unique
