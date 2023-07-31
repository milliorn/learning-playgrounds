/**
 * @param {Function} fn
 * @param {number} t
 * @return {Function}
 */
function throttle(fn, t) {
  let timeoutId = null; // stores timeout ID
  let lastArgs = []; // stores latest arguments
  let lastCallTime = 0; // stores timestamp of last call

  return function (...args) {
    const currentTime = Date.now();

    if (currentTime - lastCallTime >= t) {
      // delay has passed, call original function
      fn.apply(this, args);
      lastCallTime = currentTime;
    } else {
      // within delay period, store latest arguments
      lastArgs = args;
    }

    if (!timeoutId) {
      // timeout for next call after delay
      timeoutId = setTimeout(() => {
        if (lastArgs.length > 0) {
          // there are stored arguments, call original function with them
          fn.apply(this, lastArgs);
          lastCallTime = Date.now();
          lastArgs = []; // reset
        }
        timeoutId = null; // reset
      }, t - (currentTime - lastCallTime));
    }
  };
}
