# 125. Valid Palindrome

## Source

<https://leetcode.com/problems/valid-palindrome/description/>

A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.

## Examples

```md
Input: s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" is a palindrome.
```

```md
Input: s = "race a car"
Output: false
Explanation: "raceacar" is not a palindrome.
```

```md
Input: s = " "
Output: true
Explanation: s is an empty string "" after removing non-alphanumeric characters.
Since an empty string reads the same forward and backward, it is a palindrome.
```

## Constraints

```md
1 <= s.length <= 2 * 105
s consists only of printable ASCII characters.
```
