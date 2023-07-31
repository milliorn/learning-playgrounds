using System.Linq;

public class Solution
{
  public bool IsPalindrome(string s)
  {
    string fmtStr = new string(s.ToLower().Where(c => Char.IsLetterOrDigit(c)).ToArray());
    string reversed = new string(fmtStr.Reverse().ToArray());
    return fmtStr.Equals(reversed);
  }
}
