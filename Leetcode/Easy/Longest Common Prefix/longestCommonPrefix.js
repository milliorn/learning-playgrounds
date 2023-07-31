/**
 * @param {string[]} strs
 * @return {string}
 */
function longestCommonPrefix(strs) {
  let prefix = strs[0];

  for (const word of strs) {
    for (let i = prefix.length - 1; i >= 0; i--) {
      if (prefix[i] !== word[i]) {
        prefix = prefix.slice(0, i);
      }
    }
  }

  return prefix;
}
