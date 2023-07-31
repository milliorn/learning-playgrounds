import re

class Solution(object):
    def isPalindrome(self, s):
        formatted_str = re.sub(r'[^a-zA-Z0-9]', '', s.lower())
        reversed_str = formatted_str[::-1]
        return formatted_str == reversed_str
