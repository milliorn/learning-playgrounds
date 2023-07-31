/**
 * @param {Function} fn
 * @param {number} t
 * @return {Function}
 */
const timeLimit =
  (fn, t) =>
  async (...args) => {
    const fns = fn(...args);
    const p = new Promise((res, rej) => {
      setTimeout(() => {
        rej("Time Limit Exceeded");
      }, t);
    });

    return Promise.race([fns, p]);
  };
