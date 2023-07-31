namespace LeetCode
{
    public static class Solution
    {
        public static int StrStr(string haystack, string needle) => haystack.IndexOf(needle);

        static void Main(string[] args)
        {
            Console.WriteLine(StrStr("hello", "ll"));
            Console.WriteLine(StrStr("aaaaa", "bba"));
            Console.WriteLine(StrStr("", ""));
        }
    }
}