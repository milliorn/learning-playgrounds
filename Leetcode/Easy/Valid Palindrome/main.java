class Solution {
    public boolean isPalindrome(String s) {
        String formattedStr = s.toLowerCase().replaceAll("[^a-z0-9]", "");
        StringBuilder sb = new StringBuilder(formattedStr);
        String reversedStr = sb.reverse().toString();
        return formattedStr.equals(reversedStr);
    }
}
