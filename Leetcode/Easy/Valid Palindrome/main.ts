function isPalindrome(s: string): boolean {
  const formattedStr = s.toLowerCase().replace(/[^a-z0-9]+/g, "");
  const reversedStr = formattedStr.split("").reverse().join("");
  return formattedStr === reversedStr;
}
