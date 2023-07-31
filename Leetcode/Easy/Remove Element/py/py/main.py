class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        result = 0
        for i in nums:
            if i != val:
                nums[result] = i
                result += 1
        return result
