function isPalindromes(x: number): boolean {
  return x >= 0 ? x.toString() === x.toString().split("").reverse().join("") : false;
};