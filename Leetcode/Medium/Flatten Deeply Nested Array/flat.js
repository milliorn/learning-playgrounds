/**
 * @param {any[]} arr
 * @param {number} depth
 * @return {any[]}
 */
function flat(arr, n) {
  function flattenHelper(arr, depth) {
    if (depth >= n) {
      return arr;
    }

    let result = [];
    for (let i = 0; i < arr.length; i++) {
      if (Array.isArray(arr[i])) {
        result.push(...flattenHelper(arr[i], depth + 1));
      } else {
        result.push(arr[i]);
      }
    }
    return result;
  }

  return flattenHelper(arr, 0);
}
