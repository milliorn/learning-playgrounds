class Solution:
    def buildArray(nums):
        ans = []
        for i in range(0, len(nums)):
            ans.append(nums[nums[i]])
        return ans


solution = Solution.buildArray([0, 2, 1, 5, 3, 4])
print(solution)