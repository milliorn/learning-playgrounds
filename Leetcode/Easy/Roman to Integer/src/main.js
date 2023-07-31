/**
 * @param {string} s
 * @return {number}
 */
const numerals = {
  'I': 1,
  'V': 5,
  'X': 10,
  'L': 50,
  'C': 100,
  'D': 500,
  'M': 1000,
};

const romanToInt = s => {
  let result = numerals[ s[ s.length - 1 ] ];

  for (let i = s.length - 2; i >= 0; i--) {
    const temp = s[ i ];

    if (numerals[ temp ] >= numerals[ s[ i + 1 ] ]) {
      result += numerals[ temp ];
    }
    else {
      result -= numerals[ temp ];
    }
  }

  return result;
};

console.log(romanToInt("III"))