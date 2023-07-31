class Solution_Scala {
  def isPalindrome(x: Int): Boolean = {
    if (x < 0) return false
    val temp = Integer.toString(x)
    temp == new StringBuilder(temp).reverse.toString
  }
}