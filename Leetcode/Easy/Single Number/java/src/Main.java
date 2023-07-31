class Solution {
    public int singleNumber(int[] nums) {
        int answer = 0;

        for (int i : nums) {
            answer ^= i;
        }

        return answer;
    }
}

public class Main {
    public static void main(String[] args) {
        Solution solution = new Solution();
        int[] arr = new int[]{2, 2, 1};
        int test = solution.singleNumber(arr);
        System.out.println(test);
    }
}