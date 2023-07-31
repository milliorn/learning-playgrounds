object Solution {
  def intToRoman(num: Int) = {
    var temp = num
    val answer = new StringBuilder
    val romanInts = Array(1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1)
    val romanNumerals = Array("M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I")
    for (i <- 0 until romanInts.length) {
      while ( {
        temp >= romanInts(i)
      }) {
        temp -= romanInts(i)
        answer.append(romanNumerals(i))
      }
    }
    answer.toString
  }
}