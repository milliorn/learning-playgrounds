import java.util.Arrays;

class Solution {
    public int[] plusOne(int[] digits) {
        int[] digitsClone = digits.clone();

        for (int i = digitsClone.length - 1; i >= 0; i--) {
            if (digitsClone[i] != 9) {
                digitsClone[i]++;
                break;
            } else {
                digitsClone[i] = 0;
            }
        }

        if (digitsClone[0] == 0) {
            int[] temp = new int[digitsClone.length + 1];
            temp[0] = 1;
            return temp;
        }

        return digitsClone;
    }
}

