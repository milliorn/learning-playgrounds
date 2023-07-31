
class Solution:
    def singleNumber(self, nums: List[int]) -> List[int]:
        # Compute the XOR of all elements in the array
        xor = 0
        for num in nums:
            xor ^= num
        
        # Find the rightmost set bit of the XOR result
        rightmost_set_bit = xor & -xor
        
        # Partition the numbers into two groups based on the rightmost set bit
        group1, group2 = 0, 0

        for num in nums:
            if num & rightmost_set_bit:
                group1 ^= num
            else:
                group2 ^= num
        
        # Return the two numbers
        return [group1, group2]
