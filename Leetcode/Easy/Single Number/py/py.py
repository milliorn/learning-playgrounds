
class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        answer = 0

        for i in nums:
            answer ^= i

        return answer
