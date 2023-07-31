public class Solution {
    public boolean isPalindrome(int x) {
        if (x < 0) return false;
        String temp = Integer.toString(x);
        return temp.equals(new StringBuilder(temp).reverse().toString());
    }
}
