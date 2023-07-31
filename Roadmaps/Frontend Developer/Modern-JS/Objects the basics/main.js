let fruit = prompt("Which fruit to buy?", "apple");

let bag = {
  [fruit]: 5, // the name of the property is taken from the variable fruit
};

alert(bag.apple); // 5 if fruit="apple"

function makeUser(name, age) {
  return {
    name: name,
    age: age,
    // ...other properties
  };
}

let user = makeUser("John", 30);
alert(user.name); // John
alert(user.age);

let obj = {
  for: 1,
  let: 2,
  return: 3,
};

alert(obj.for + obj.let + obj.return); // 6

obj = {
  0: "test", // same as "0": "test"
};

// both alerts access the same property (the number 0 is converted to string
// "0")
alert(obj["0"]); // test
alert(obj[0]); // test (same property)

user = {};

alert(user.noSuchProperty === undefined); // true means "no such property"

user = {
  name: "John",
  age: 30,
};

alert("age" in user); // true, user.age exists
alert("blabla" in user); // false, user.blabla doesn't exist

user = {
  name: "John",
  age: 30,
  isAdmin: true,
  sayName() {
    alert(this.name);
  },
};

for (let key in user) {
  // keys
  alert(key); // name, age, isAdmin
  // values for the keys
  alert(user[key]); // John, 30, true
}

user.sayHi = () => alert("Hello!");
user.sayHi();
user.sayName();
alert(user.address && user.address.street && user.address.street.name); // undefined (no error)
