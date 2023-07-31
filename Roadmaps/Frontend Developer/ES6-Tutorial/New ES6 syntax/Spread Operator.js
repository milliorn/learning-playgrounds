const odd = [1, 3, 5];
const combined = [2, 4, 6, ...odd];
console.log(combined);

function f(a, b, ...args) {
  console.log(args);
}

f(1, 2, 3, 4, 5);

function compare(a, b) {
  return a - b;
}

let result = compare(...[1, 2]);
console.log(result); // -1

let rivers = ["Nile", "Ganges", "Yangtze"];
let moreRivers = ["Danube", "Amazon"];

rivers.push(...moreRivers);
console.log(rivers);

let initialChars = ["A", "B"];
let chars = [...initialChars, "C", "D"];
console.log(chars); // ["A", "B", "C", "D"]

let scores = [80, 70, 90];
let copiedScores = [...scores];
console.log(copiedScores); // [80, 70, 90]

chars = ["A", ..."BC", "D"];
console.log(chars); // ["A", "B", "C", "D"]
