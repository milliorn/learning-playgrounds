public class Solution {
  public String addBinary(String a, String b) {
    StringBuilder result = new StringBuilder();
    int rem = Calculate(a, b, result, a.length() - 1, b.length() - 1, 0);

    if (rem > 0) {
      result.insert(0, rem);
    }

    return result.toString();
  }

  private int Calculate(String a, String b, StringBuilder result, int sizeA, int sizeB, int carry) {
    while (sizeA >= 0 || sizeB >= 0) {
      int temp = carry;

      if (sizeA >= 0) {
        temp += a.charAt(sizeA--) - '0';
      }
      if (sizeB >= 0) {
        temp += b.charAt(sizeB--) - '0';
      }

      result.insert(0, temp % 2);
      carry = temp / 2;
    }
    return carry;
  }
}
