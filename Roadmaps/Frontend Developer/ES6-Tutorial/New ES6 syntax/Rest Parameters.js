/*jshint -W054 */

function sum(...args) {
  let total = 0;
  for (const a of args) {
    total += a;
  }
  return total;
}

console.log(sum(1, 2, 3));

let result = sum(10, "Hi", null, undefined, 20);
console.log(result);

function sum(...args) {
  return args
    .filter(function (e) {
      return typeof e === "number";
    })
    .reduce(function (prev, curr) {
      return prev + curr;
    });
}

result = sum(10, "Hi", null, undefined, 20);
console.log(result);

function sum() {
  return Array.prototype.filter
    .call(arguments, function (e) {
      return typeof e === "number";
    })
    .reduce(function (prev, curr) {
      return prev + curr;
    });
}

console.log(sum(1, 2, 3));

result = sum(10, "Hi", null, undefined, 20);
console.log(result);

const showNumbers = new Function("...numbers", "console.log(numbers)");
console.log(showNumbers(1, 2, 3));
