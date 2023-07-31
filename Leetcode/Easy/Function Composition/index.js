/**
 * @param {Function[]} functions
 * @return {Function}
 */
const compose = (functions) => {
  // return identity function
  if (functions.length === 0) {
    return (x) => x;
  }

  return (x) => {
    // start with right-most function
    let result = functions[functions.length - 1](x);

    // apply each function right to left
    for (let i = functions.length - 2; i >= 0; i--) {
      result = functions[i](result);
    }

    return result;
  };
};
