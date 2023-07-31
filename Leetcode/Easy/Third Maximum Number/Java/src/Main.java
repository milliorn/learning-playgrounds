class Solution {
    public static void main(String[] args) {
        System.out.println("Hello world!");
    }

    public int thirdMax(int[] nums) {
        int max1 = Integer.MIN_VALUE;
        int max2 = Integer.MIN_VALUE;
        int max3 = Integer.MIN_VALUE;
        boolean hasThirdMax = false;

        for (int num : nums) {
            if (num == max1 || num == max2 || num == max3) {
                continue;
            }

            if (num > max1) {
                max3 = max2;
                max2 = max1;
                max1 = num;
            } else if (num > max2) {
                max3 = max2;
                max2 = num;
            } else if (num > max3) {
                max3 = num;
            }

            if (!hasThirdMax && max3 != Integer.MIN_VALUE) {
                hasThirdMax = true;
            }
        }

        return hasThirdMax ? max3 : max1;
    }
}