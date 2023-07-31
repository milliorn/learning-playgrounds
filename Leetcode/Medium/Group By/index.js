Array.prototype.groupBy = (fn) => {
  const result = {};
  for (let i = 0; i < this.length; i++) {
    const key = fn(this[i]);
    if (result[key]) {
      result[key].push(this[i]);
    } else {
      result[key] = [this[i]];
    }
  }
  return result;
};

/**
 * const array = [
 *   {"id": "1"},
 *   {"id": "1"},
 *   {"id": "2"}
 * ];
 * const fn = function(item) {
 *   return item.id;
 * };
 * array.groupBy(fn);
 * // Output: { "1": [{"id": "1"}, {"id": "1"}], "2": [{"id": "2"}] }
 */
