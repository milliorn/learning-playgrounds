class Solution:
    @staticmethod
    def lengthOfLastWord(s: str) -> int:
        x = s.split()
        return len(x[-1])


if __name__ == '__main__':
    print(Solution.lengthOfLastWord("hello world"))
    print(Solution.lengthOfLastWord("   fly me   to   the moon  "))
    print(Solution.lengthOfLastWord("luffy is still joyboy"))
