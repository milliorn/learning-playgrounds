class Solution {

  /**
  * @param String $s
  * @return Boolean
  */
  function isPalindrome(string $s): bool {
      $lowercase = strtolower($s);
      $formatted = preg_replace('/[^a-zA-Z0-9]/', '', $lowercase);
      return strrev($formatted) === $formatted;
  }
}