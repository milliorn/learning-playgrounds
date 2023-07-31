class Solution(object):
    def checkPerfectNumber(self, num):
        if num <= 1:
            return False

        divisorSum = 1
        divisor = 2

        while divisor * divisor <= num:
            if num % divisor == 0:
                divisorSum += divisor
                if divisor != num // divisor:
                    divisorSum += num // divisor
            divisor += 1

        return divisorSum == num
