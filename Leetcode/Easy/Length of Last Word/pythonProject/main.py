class Solution:
    @staticmethod
    def addBinary(a: str, b: str) -> str:
        length = max(len(a), len(b))
        answer = ''
        rem = 0
        answer, rem = Solution.calculate(a.zfill(length), answer, b.zfill(length), length, rem)
        if rem != 0:
            answer = '1' + answer
        return answer.zfill(length)

    @staticmethod
    def calculate(a, answer, b, length, rem):
        for i in range(length - 1, -1, -1):
            r = rem
            r += 1 if a[i] == '1' else 0
            r += 1 if b[i] == '1' else 0
            answer = ('1' if r % 2 == 1 else '0') + answer
            rem = 0 if r < 2 else 1
        return answer, rem


def print_hi(name):
    print(f'Hi, {name}')  # Press Ctrl+F8 to toggle the breakpoint.


if __name__ == '__main__':
    print_hi('PyCharm')
    print(Solution.addBinary("1"))
