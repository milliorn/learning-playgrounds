public class Main {
    public static void main(String[] args) {
        System.out.println("Hello world!");
    }

    public int[] singleNumber(int[] nums) {
        /* Compute the XOR of all elements in the array */
        int xor = 0;

        for (int i = 0; i < nums.length; i++) {
            xor ^= nums[i];
        }

        /* Find the rightmost set bit of the XOR result */
        int rightmost_set_bit = xor & -xor;

        /* Partition the numbers into two groups based on the rightmost set bit */
        int[] answer = {0, 0};

        for (int i = 0; i < nums.length; i++) {
            int num = nums[i];

            if ((num & rightmost_set_bit) == 0) {
                answer[0] ^= num;
            } else {
                answer[1] ^= num;
            }
        }

        return answer;
    }
}