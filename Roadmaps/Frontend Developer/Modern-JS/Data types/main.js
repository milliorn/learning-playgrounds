let map = new Map();

map.set("1", "str1");
map.set(1, "num1");
map.set(true, "bool1");

console.log(map.get(1));
console.log(map.get("1"));
console.log(map.size);

let john = { name: "John" };
let visitsCountMap = new Map();

visitsCountMap.set(john, 123);

console.log(visitsCountMap.get(john));

let ben = { name: "Ben" };
let visitCountObj = {};

visitCountObj[ben] = 234;
visitCountObj[john] = 123;

console.log(visitCountObj["[object Object]"]);

map.set("2", "str2").set(2, "num2").set(false, "bool2");

let recipeMap = new Map([
  ["cucumber", 500],
  ["tomatoes", 350],
  ["onion", 50],
]);

for (let vegetable of recipeMap.keys()) {
  console.log(vegetable); // cucumber, tomatoes, onion
}

for (let amount of recipeMap.values()) {
  console.log(amount); // 500, 350, 50
}

for (let entry of recipeMap) {
  console.log(entry); // cucumber,500 (and so on)
}

recipeMap.forEach((value, key, map) => {
  console.log(`${key}: ${value}`); // cucumber: 500 etc
});

let map1 = new Map([
  ["1", "str1"],
  [1, "num1"],
  [true, "bool1"],
]);

console.log(map1.get("1")); // str1

let prices = Object.fromEntries([
  ["banana", 1],
  ["orange", 2],
  ["meat", 4],
]);

console.log(prices.orange); // 2

let set = new Set();

let joe = { name: "Jpe" };
let pete = { name: "Pete" };
let mary = { name: "Mary" };

set.add(joe);
set.add(pete);
set.add(mary);
set.add(joe);
set.add(mary);

console.log(set.size); // 3

for (let user of set) {
  console.log(user.name); // joe (then Pete and Mary)
}

let jack = { name: "John" };

let weakMap = new WeakMap();
weakMap.set(jack, "...");

jack = null; // overwrite the reference

// jack is removed from memory!

visitsCountMap = new Map(); // map: user => visits count

// increase the visits count
function countUser(user) {
  let count = visitsCountMap.get(user) || 0;
  visitsCountMap.set(user, count + 1);
}

john = {
  name: "John",
};

countUser(john); // count his visits

// later john leaves us
john = null;

let cache = new Map();

// calculate and remember the result
function process(obj) {
  if (!cache.has(obj)) {
    let result = /* calculations of the result for */ obj;

    cache.set(obj, result);
  }

  return cache.get(obj);
}

// Now we use process() in another file:

// :file_folder: main.js
let obj = {
  /* let's say we have an object */
};

let result1 = process(obj); // calculated

// ...later, from another place of the code...
let result2 = process(obj); // remembered result taken from cache

// ...later, when the object is not needed any more:
obj = null;

console.log(cache.size); // 1 (Ouch! The object is still in cache, taking memory!)

let visitedSet = new WeakSet();

john = {
  name: "John",
};
pete = {
  name: "Pete",
};
mary = {
  name: "Mary",
};

visitedSet.add(john); // John visited us
visitedSet.add(pete); // Then Pete
visitedSet.add(john); // John again

// visitedSet has 2 users now

// check if John visited?
console.log(visitedSet.has(john)); // true

// check if Mary visited?
console.log(visitedSet.has(mary)); // false

john = null;

// visitedSet will be cleaned automatically

prices = {
  banana: 1,
  orange: 2,
  meat: 4,
};

let doublePrices = Object.fromEntries(
  // convert prices to array, map each key/value pair into another pair
  // and then fromEntries gives back the object
  Object.entries(prices).map((entry) => [entry[0], entry[1] * 2])
);

console.log(doublePrices.meat); // 8

// reduce loops over array of salaries,
// adding them up
// and returns the result
const sumSalaries = (salaries) =>
  Object.values(salaries).reduce((a, b) => a + b, 0); // 650

let salaries = {
  John: 100,
  Pete: 300,
  Mary: 250,
};

console.log(sumSalaries(salaries)); // 650

const count = (obj) => Object.keys(obj).length;

let user = {
  name: "John",
  age: 30,
};

console.log(count(user)); // 2

// we have an array with the name and surname
let arr = ["John", "Smith"];

// destructuring assignment
// sets firstName = arr[0]
// and surname = arr[1]
let [firstName, surname] = arr;

console.log(firstName); // John
console.log(surname); // Smith

// second element is not needed
[firstName, , title] = ["Julius", "Caesar", "Consul", "of the Roman Republic"];

console.log(title); // Consul

let [a, b, c] = "abc"; // ["a", "b", "c"]
let [one, two, three] = new Set([1, 2, 3]);

let guest = "Jane";
let admin = "Pete";

// Let's swap the values: make guest=Pete, admin=Jane
[guest, admin] = [admin, guest];

console.log(`${guest} ${admin}`); // Pete Jane (successfully swapped!)

let [name1, name2] = ["Julius", "Caesar", "Consul", "of the Roman Republic"];

console.log(name1); // Julius
console.log(name2); // Caesar
// Further items aren't assigned anywhere

let [name3, name4, ...rest] = [
  "Julius",
  "Caesar",
  "Consul",
  "of the Roman Republic",
];

// rest is array of items, starting from the 3rd one
console.log(rest[0]); // Consul
console.log(rest[1]); // of the Roman Republic
console.log(rest.length); // 2

// default values
[name = "Guest", surname = "Anonymous"] = ["Julius"];

console.log(name); // Julius (from array)
console.log(surname); // Anonymous (default used)

let options = {
  title: "Menu",
  width: 100,
  height: 200,
};

let { title1, width1, height1 } = options;

console.log(title1); // Menu
console.log(width1); // 100
console.log(height1); // 200

// changed the order in let {...}
let { height2, width2, title2 } = { title: "Menu", height: 200, width: 100 };

let options1 = {
  title: "Menu",
};

let { width3 = 100, height3 = 200, title3 } = options;

console.log(title3); // Menu
console.log(width3); // 100
console.log(height3); // 200

let options2 = {
  size: {
    width: 100,
    height: 200,
  },
  items: ["Cake", "Donut"],
  extra: true,
};

// destructuring assignment split in multiple lines for clarity
let {
  size: {
    // put size here
    width,
    height,
  },
  items: [item1, item2], // assign items here
  title4 = "Menu", // not present in the object (default value is used)
} = options2;

console.log(title4); // Menu
console.log(width); // 100
console.log(height); // 200
console.log(item1); // Cake
console.log(item2); // Donut

let now = new Date();
console.log(now); // shows current date/time

// 0 means 01.01.1970 UTC+0
let Jan01_1970 = new Date(0);
console.log(Jan01_1970);

// now add 24 hours, get 02.01.1970 UTC+0
let Jan02_1970 = new Date(24 * 3600 * 1000);
console.log(Jan02_1970);

// 31 Dec 1969
let Dec31_1969 = new Date(-24 * 3600 * 1000);
console.log(Dec31_1969);

let date = new Date(2013, 0, 32); // 32 Jan 2013 ?!?
console.log(date); // ...is 1st Feb 2013!

// the hour in your current time zone
console.log(date.getHours());

// the hour in UTC+0 time zone (London time without daylight savings)
console.log(date.getUTCHours());

let today = new Date();

today.setHours(0);
console.log(today); // still today, but the hour is changed to 0

today.setHours(0, 0, 0, 0);
console.log(today); // still today, now 00:00:00 sharp.

date = new Date();
console.log(+date); // the number of milliseconds, same as date.getTime()

let start = Date.now(); // milliseconds count from 1 Jan 1970

// do the job
for (let i = 0; i < 100000; i++) {
  let doSomething = i * i * i;
}

let end = Date.now(); // done

console.log(`The loop took ${end - start} ms`); // subtract numbers, not dates

function diffSubtract(date1, date2) {
  return date2 - date1;
}

function diffGetTime(date1, date2) {
  return date2.getTime() - date1.getTime();
}

function bench(f) {
  let date1 = new Date(0);
  let date2 = new Date();

  let start = Date.now();
  for (let i = 0; i < 100000; i++) f(date1, date2);
  return Date.now() - start;
}

let time1 = 0;
let time2 = 0;

// run bench(diffSubtract) and bench(diffGetTime) each 10 times alternating
for (let i = 0; i < 10; i++) {
  time1 += bench(diffSubtract);
  time2 += bench(diffGetTime);
}

console.log("Total time for diffSubtract: " + time1);
console.log("Total time for diffGetTime: " + time2);

// added for "heating up" prior to the main loop
bench(diffSubtract);
bench(diffGetTime);

// now benchmark
for (let i = 0; i < 10; i++) {
  time1 += bench(diffSubtract);
  time2 += bench(diffGetTime);
}

bench(diffSubtract);
bench(diffGetTime);

console.log("Total time for diffSubtract: " + time1);
console.log("Total time for diffGetTime: " + time2);

user = {
  name: "John",
  age: 30,

  toString() {
    return `{name: "${this.name}", age: ${this.age}}`;
  },
};

alert(user); // {name: "John", age: 30}

let student = {
  name: "John",
  age: 30,
  isAdmin: false,
  courses: ["html", "css", "js"],
  wife: null,
};

let json = JSON.stringify(student);

let room = {
  number: 23,
};

let meetup = {
  title: "Conference",
  participants: [{ name: "John" }, { name: "Alice" }],
  place: room, // meetup references room
};

room.occupiedBy = meetup; // room references meetup

alert(JSON.stringify(meetup, ["title", "participants"]));
// {"title":"Conference","participants":[{},{}]}

let room = {
  number: 23,
};

meetup = {
  title: "Conference",
  participants: [{ name: "John" }, { name: "Alice" }],
  place: room, // meetup references room
};

room.occupiedBy = meetup; // room references meetup

alert(
  JSON.stringify(meetup, (key, value) => {
    alert(`${key}: ${value}`);
    return key == "occupiedBy" ? undefined : value;
  })
);

/* key:value pairs that come to replacer:
:             [object Object]
title:        Conference
participants: [object Object],[object Object]
0:            [object Object]
name:         John
1:            [object Object]
name:         Alice
place:        [object Object]
number:       23
occupiedBy: [object Object]
*/

user = {
  name: "John",
  age: 25,
  roles: {
    isAdmin: false,
    isEditor: true,
  },
};

alert(JSON.stringify(user, null, 2));
/* two-space indents:
{
  "name": "John",
  "age": 25,
  "roles": {
    "isAdmin": false,
    "isEditor": true
  }
}
*/

/* for JSON.stringify(user, null, 4) the result would be more indented:
{
    "name": "John",
    "age": 25,
    "roles": {
        "isAdmin": false,
        "isEditor": true
    }
}
*/

// stringified array
let numbers = "[0, 1, 2, 3]";

numbers = JSON.parse(numbers);

alert(numbers[1]); // 1
