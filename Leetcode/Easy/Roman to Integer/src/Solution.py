class Solution:
    def romanToInt(self, s: str) -> int:
        return self.get_roman(self.roman_numerals(), s)

    @staticmethod
    def roman_numerals():
        numerals = {
            'I': 1,
            'V': 5,
            'X': 10,
            'L': 50,
            'C': 100,
            'D': 500,
            'M': 1000,
        }
        return numerals

    @staticmethod
    def get_roman(numerals, s):
        temp = None
        result = 0
        for i in s:
            if temp and numerals[i] > temp:
                result -= temp * 2
            else:
                temp = numerals[i]
            result += numerals[i]
        return result
