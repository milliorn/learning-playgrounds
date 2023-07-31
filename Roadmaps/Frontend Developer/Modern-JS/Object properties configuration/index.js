let user = {
  name: "John",
};

let descriptor = Object.getOwnPropertyDescriptor(user, "name");

console.log(JSON.stringify(descriptor, null, 2));
/* property descriptor:
{
  "value": "John",
  "writable": true,
  "enumerable": true,
  "configurable": true
}
*/

user = {};

Object.defineProperty(user, "name", {
  value: "John",
});

descriptor = Object.getOwnPropertyDescriptor(user, "name");

console.log(JSON.stringify(descriptor, null, 2));
/*
{
  "value": "John",
  "writable": false,
  "enumerable": false,
  "configurable": false
}
 */

user = {
  name: "John",
};

Object.defineProperty(user, "name", {
  writable: false,
});

user.name = "Pete"; // Error: Cannot assign to read only property 'name'

user = {};

Object.defineProperty(user, "name", {
  value: "John",
  // for new properties we need to explicitly list what's true
  enumerable: true,
  configurable: true,
});

console.log(user.name); // John
user.name = "Pete"; // Error

user = {
  name: "John",
  toString() {
    return this.name;
  },
};

// By default, both our properties are listed:
for (let key in user) console.log(key); // name, toString

user = {
  name: "John",
  toString() {
    return this.name;
  },
};

Object.defineProperty(user, "toString", {
  enumerable: false,
});

// Now our toString disappears:
for (let key in user) console.log(key); // name

descriptor = Object.getOwnPropertyDescriptor(Math, "PI");

console.log(JSON.stringify(descriptor, null, 2));
/*
{
  "value": 3.141592653589793,
  "writable": false,
  "enumerable": false,
  "configurable": false
}
*/

user = {
  name: "John",
  surname: "Smith",

  get fullName() {
    return `${this.name} ${this.surname}`;
  },

  set fullName(value) {
    [this.name, this.surname] = value.split(" ");
  },
};

// set fullName is executed with the given value.
user.fullName = "Alice Cooper";

console.log(user.name); // Alice
console.log(user.surname); // Cooper

user = {
  get name() {
    return this._name;
  },

  set name(value) {
    if (value.length < 4) {
      console.log("Name is too short, need at least 4 characters");
      return;
    }
    this._name = value;
  },
};

user.name = "Pete";
console.log(user.name); // Pete

user.name = ""; // Name is too short...

/**function User(name, birthday) {
  this.name = name;
  this.birthday = birthday;

  // age is calculated from the current date and birthday
  Object.defineProperty(this, "age", {
    get() {
      let todayYear = new Date().getFullYear();
      return todayYear - this.birthday.getFullYear();
    },
  });
} 
*/

class User {
  constructor(name, birthday) {
    this.name = name;
    this.birthday = birthday;

    // age is calculated from the current date and birthday
    Object.defineProperty(this, "age", {
      get() {
        let todayYear = new Date().getFullYear();
        return todayYear - this.birthday.getFullYear();
      },
    });
  }
}

let john = new User("John", new Date(1992, 6, 1));

console.log(john.birthday); // birthday is available
console.log(john.age); // ...as well as the age
