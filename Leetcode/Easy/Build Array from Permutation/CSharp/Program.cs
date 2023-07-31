public class Solution
{
    public int[] BuildArray(int[] nums)
    {
        List<int> ans = new List<int>();

        for (int i = 0; i < nums.Length; ++i)
        {
            ans.Add(nums[nums[i]]);
        }

        return ans.ToArray();
    }

    public static void Main(string[] args)
    {
        Solution solution = new Solution();
        int[] testArr = new int[] { 0, 2, 1, 5, 3, 4 };
        int[] test = solution.BuildArray(testArr);

        foreach (int i in testArr) { Console.Write(i.ToString()); }
        Console.WriteLine('\n');
        foreach (int i in test) { Console.Write(i.ToString()); }
    }
}