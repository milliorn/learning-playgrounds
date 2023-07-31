function sumAll(...args) {
  // args is the name for the array
  let sum = 0;

  for (let arg of args) sum += arg;

  return sum;
}

console.log(sumAll(1)); // 1
console.log(sumAll(1, 2)); // 3
console.log(sumAll(1, 2, 3)); // 6

function showName(firstName, lastName, ...titles) {
  console.log(firstName + " " + lastName); // Julius Caesar

  // the rest go into titles array
  // i.e. titles = ["Consul", "Imperator"]
  console.log(titles[0]); // Consul
  console.log(titles[1]); // Imperator
  console.log(titles.length); // 2
}

showName("Julius", "Caesar", "Consul", "Imperator");

function showName() {
  console.log(arguments.length);
  console.log(arguments[0]);
  console.log(arguments[1]);

  // it's iterable
  // for(let arg of arguments) console.log(arg);
}

// shows: 2, Julius, Caesar
showName("Julius", "Caesar");

// shows: 1, Ilea, undefined (no second argument)
showName("Ilea");

let arr = [3, 5, 1];
let arr1 = [1, -2, 3, 4];
let arr2 = [8, 3, -8, 1];

console.log(Math.max(...arr1)); // 5 (spread turns array into a list of arguments)

console.log(Math.max(...arr1, ...arr2)); // 8

arr1 = [3, 5, 1];
arr2 = [8, 9, 15];

let merged = [0, ...arr1, 2, ...arr2];

console.log(merged); // 0,3,5,1,2,8,9,15 (0, then arr, then 2, then arr2)

let str = "Hello";

console.log([...str]); // H,e,l,l,o

arr1 = [1, 2, 3];

let arrCopy = [...arr1]; // spread the array into a list of parameters
// then put the result into a new array

// do the arrays have the same contents?
console.log(JSON.stringify(arr1) === JSON.stringify(arrCopy)); // true

// are the arrays equal?
console.log(arr1 === arrCopy); // false (not same reference)

// modifying our initial array does not modify the copy:
arr1.push(4);
console.log(arr1); // 1, 2, 3, 4
console.log(arrCopy); // 1, 2, 3

function slow(x) {
  // there can be a heavy CPU-intensive job here
  console.log(`Called with ${x}`);
  return x;
}

function cachingDecorator(func) {
  let cache = new Map();

  return (x) => {
    if (cache.has(x)) {
      // if there's such key in cache
      return cache.get(x); // read the result from it
    }

    let result = func(x); // otherwise call func

    cache.set(x, result); // and cache (remember) the result
    return result;
  };
}

slow = cachingDecorator(slow);

console.log(slow(1)); // slow(1) is cached and the result returned
console.log("Again: " + slow(1)); // slow(1) result returned from cache

console.log(slow(2)); // slow(2) is cached and the result returned
console.log("Again: " + slow(2)); // slow(2) result returned from cache

let worker = {
  slow(min, max) {
    console.log(`Called with ${min},${max}`);
    return min + max;
  },
};

function hash(args) {
  return args[0] + "," + args[1];
}

function cachingDecorator(func, _hash) {
  let cache = new Map();
  return function () {
    let key = hash(arguments); // (*)
    if (cache.has(key)) {
      return cache.get(key);
    }

    let result = func.call(this, ...arguments); // (**)

    cache.set(key, result);
    return result;
  };
}

worker.slow = cachingDecorator(worker.slow, hash);

console.log(worker.slow(3, 5)); // works
console.log("Again " + worker.slow(3, 5)); // same (cached)

let user = {
  firstName: "John",
  sayHi() {
    console.log(`Hello, ${this.firstName}!`);
  },
};

let sayHi = user.sayHi.bind(user); // (*)

// can run it without an object
sayHi(); // Hello, John!

setTimeout(sayHi, 1000); // Hello, John!

// even if the value of user changes within 1 second
// sayHi uses the pre-bound value which is reference to the old user object
user = {
  sayHi() {
    console.log("Another user in setTimeout!");
  },
};

function partial(func, ...argsBound) {
  return function (...args) {
    // (*)
    return func.call(this, ...argsBound, ...args);
  };
}

// Usage:
user = {
  firstName: "John",
  say(time, phrase) {
    console.log(`[${time}] ${this.firstName}: ${phrase}!`);
  },
};

// add a partial method with fixed time
user.sayNow = partial(
  user.say,
  new Date().getHours() + ":" + new Date().getMinutes()
);

user.sayNow("Hello");
// Something like:
// [10:00] John: Hello!
