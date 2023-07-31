class Solution {
    public String intToRoman(int num) {
        StringBuilder answer = new StringBuilder();
        int[] romanInts = {1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1};
        String[] romanNumerals = {
                "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"
        };

        for (int i = 0; i < romanInts.length; i++) {
            while (num >= romanInts[i]) {
                num -= romanInts[i];
                answer.append(romanNumerals[i]);
            }
        }
        return answer.toString();
    }
}
