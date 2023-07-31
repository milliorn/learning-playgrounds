class User {
  constructor(name) {
    this.name = name;
  }
  sayHi() {
    console.log(this.name);
  }
}

let user = new User("John");
user.sayHi();

// class is a function
console.log(typeof User); // function

// ...or, more precisely, the constructor method
console.log(User === User.prototype.constructor); // true

// The methods are in User.prototype, e.g:
console.log(User.prototype.sayHi); // the code of the sayHi method

// there are exactly two methods in the prototype
console.log(Object.getOwnPropertyNames(User.prototype)); // constructor, sayHi

// rewriting class User in pure functions

// 1. Create constructor function
function Using(name) {
  this.name = name;
}
// a function prototype has "constructor" property by default,
// so we don't need to create it

// 2. Add the method to prototype
Using.prototype.sayHi = function () {
  console.log(this.name);
};

// Usage:
user = new Using("John");
user.sayHi();

// "Named Class Expression"
// (no such term in the spec, but that's similar to Named Function Expression)
let Users = class User {
  sayHi() {
    console.log(User); // MyClass name is visible only inside the class
  }
};

new Users().sayHi(); // works, shows MyClass definition

console.log(User); // error, MyClass name isn't visible outside of the class

class UserTest {
  constructor(name) {
    // invokes the setter
    this.name = name;
  }

  get name() {
    return this._name;
  }

  set name(value) {
    if (value.length < 4) {
      console.log("Name is too short.");
      return;
    }
    this._name = value;
  }
}

let test = new UserTest("John");
console.log(user.name); // John

test = new UserTest(""); // Name is too short.

class Animal {
  constructor(name) {
    this.speed = 0;
    this.name = name;
  }

  run(speed) {
    this.speed = speed;
    console.log(`${this.name} runs with speed ${this.speed}.`);
  }

  stop() {
    this.speed = 0;
    console.log(`${this.name} stands still.`);
  }
}

class Rabbit extends Animal {
  hide() {
    console.log(`${this.name} hides!`);
  }

  stop() {
    super.stop(); // call parent stop
    this.hide(); // and then hide
  }
}

let rabbit = new Rabbit("White Rabbit");

rabbit.run(5); // White Rabbit runs with speed 5.
rabbit.hide(); // White Rabbit hides!

function f(phrase) {
  return class {
    sayHi() {
      console.log(phrase);
    }
  };
}

class Uses extends f("Hello") {}

new Uses().sayHi(); // Hello
