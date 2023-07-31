class Solution(object):
    def twoSum(self, nums, target):
        numbers = dict()

        for key, value in enumerate(nums):
            numbers[value] = key

        solution = list()

        for index in range(len(nums)):
            value = nums[index]
            total = target - value
            temp = numbers.get(total, None)

            if temp is not None and temp != index:
                solution = list([index, temp])

        return solution
