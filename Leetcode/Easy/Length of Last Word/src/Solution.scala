class Solution {
  def lengthOfLastWord(s: String): Int = {
    var result = 0
    val ss = s.trim
    for (i <- ss.length - 1 to 0 by -1) {
      if (ss.charAt(i) == ' ') return result
      else result += 1
    }
    0
  }
}