class Solution {
    public String longestCommonPrefix(String[] strs) {
        StringBuilder prefix = new StringBuilder(strs[0]);
        int position = 0;
        int maxCharIndex = strs[0].length();

        for (int i = 1; i < strs.length; ++i) {
            maxCharIndex = Math.min(maxCharIndex, strs[i].length());
        }

        while (position < maxCharIndex) {
            char previousLetter = strs[0].charAt(position);

            for (int i = 1; i < strs.length; ++i) {
                if (previousLetter == strs[i].charAt(position)) {
                    continue;
                }
                return prefix.substring(0, position);
            }
            ++position;
            prefix.append(previousLetter);
        }
        return prefix.substring(0, position);
    }
}

public class Main {
    public static void main(String[] args) {
        System.out.println("Hello world!");
    }
}