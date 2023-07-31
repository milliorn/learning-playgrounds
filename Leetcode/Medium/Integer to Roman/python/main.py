class Solution:
    def intToRoman(self, num: int) -> str:
        answer = ""
        roman_ints = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1]
        roman_numerals = ['M', 'CM', 'D', 'CD', 'C', 'XC', 'L', 'XL', 'X', 'IX', 'V', 'IV', 'I']
        for i in range(len(roman_ints)):
            while num >= roman_ints[i]:
                num -= roman_ints[i]
                answer += roman_numerals[i]
        return answer


def print_hi(name):
    print(f'Hi, {name}')  # Press Ctrl+F8 to toggle the breakpoint.


if __name__ == '__main__':
    print_hi('PyCharm')
