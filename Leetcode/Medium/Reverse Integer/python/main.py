class Solution:
    def reverse(self, x: int) -> int:
        result = int(str(x)[::-1]) if x > 0 else -int(str(abs(x))[::-1])
        return 0 if (result < -2147483648 or result > 2147483647) else result


def print_hi(name):
    print(f'Hi, {name}')  # Press Ctrl+F8 to toggle the breakpoint.


if __name__ == '__main__':
    print_hi('PyCharm')
