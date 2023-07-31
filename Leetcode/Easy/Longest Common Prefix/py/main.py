class Solution(object):
    def longestCommonPrefix(self, strs):
        prefix = ""
        for position in range(len(strs[0])):
            for word in strs:
                if position == len(word) or word[position] != strs[0][position]:
                    return prefix
            prefix += strs[0][position]
        return prefix
