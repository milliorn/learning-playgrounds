class Solution(object):
    @staticmethod
    def isPalindrome(x):
        if x < 0:
            return False
        temp = str(x)
        return temp == temp[::-1]


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    print(Solution.isPalindrome(121))
    print(Solution.isPalindrome(-121))
    print(Solution.isPalindrome(10))
