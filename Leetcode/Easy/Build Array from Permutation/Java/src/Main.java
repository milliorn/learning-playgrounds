import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        System.out.println("Hello world!");
    }

    public int[] buildArray(int[] nums) {
        List ans = new ArrayList();

        for (int i = 0; i < nums.length; i++) ans.add(nums[nums[i]]);

        return ans.stream().mapToInt(i -> (int) i).toArray();
    }
}