const numeral: { [ key: string ]: number } = {
  'I': 1,
  'V': 5,
  'X': 10,
  'L': 50,
  'C': 100,
  'D': 500,
  'M': 1000,
};

function romanToInts(s: string): number {
  let result: number = numeral[ s[ s.length - 1 ] ];

  for (let i: number = s.length - 2; i >= 0; i--) {
    const temp: string = s[ i ];

    if (numeral[ temp ] >= numeral[ s[ i + 1 ] ]) {
      result += numeral[ temp ];
    }
    else {
      result -= numeral[ temp ];
    }
  }

  return result;
}; 