class Solution:
    def mySqrt(self, x: int) -> int:
        result = 1.0
        while abs(result * result - x) > 0.1:
            result = (result + x / result) / 2
        return int(result)
