import java.util.Arrays;

public class Main {
    public int[] getConcatenation(int[] nums) {
        int[] nums_copy = nums.clone();
        int[] both = Arrays.copyOf(nums, nums.length + nums_copy.length);
        System.arraycopy(nums_copy, 0, both, nums.length, nums_copy.length);
        return both;
    }

    public static void main(String[] args) {
        System.out.println("Hello world!");
    }
}