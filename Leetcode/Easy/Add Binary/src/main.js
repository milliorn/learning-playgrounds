/**
 * @param {string} a
 * @param {string} b
 * @return {string}
 */
const addBinary = (a, b) => {
  let result = "";
  let tempA = a.length - 1;
  let tempB = b.length - 1;
  const rem = calculate();

  if (rem > 0) {
    result = 1 + result;
  }

  return result;

  function calculate() {
    let rem = 0;

    while (tempA >= 0 || tempB >= 0) {
      let sum = rem;

      if (tempA >= 0) {
        sum += a[ tempA-- ] - '0';
      }
      if (tempB >= 0) {
        sum += b[ tempB-- ] - '0';
      }

      result = sum % 2 + result;
      rem = parseInt(sum / 2);
    }
    return rem;
  }
};
