class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        return len(nums) if len(nums) < 2 else self.calculate(nums)

    @staticmethod
    def calculate(nums):
        result = 0
        for i in range(1, len(nums)):
            if nums[i] != nums[result]:
                result += 1
                nums[result] = nums[i]
        return result + 1
