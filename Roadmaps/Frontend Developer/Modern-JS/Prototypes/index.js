/* jshint proto: true */

let user = {
  name: "John",
  surname: "Smith",

  set fullName(value) {
    [this.name, this.surname] = value.split(" ");
  },

  get fullName() {
    return `${this.name} ${this.surname}`;
  },
};

let admin = {
  __proto__: user,
  isAdmin: true,
};

console.log(admin.fullName); // John Smith (*)

// setter triggers!
admin.fullName = "Alice Cooper"; // (**)

console.log(admin.fullName); // Alice Cooper, state of admin modified
console.log(user.fullName); // John Smith, state of user protected

// animal has methods
let animal = {
  walk() {
    if (!this.isSleeping) {
      console.log(`I walk`);
    }
  },
  sleep() {
    this.isSleeping = true;
  },
};

let rabbit = {
  name: "White Rabbit",
  __proto__: animal,
};

// modifies rabbit.isSleeping
rabbit.sleep();

console.log(rabbit.isSleeping); // true
console.log(animal.isSleeping); // undefined (no such property in the prototype)

animal = {
  eats: true,
};

rabbit = {
  jumps: true,
  __proto__: animal,
};

// Object.keys only returns own keys
console.log(Object.keys(rabbit)); // jumps

// for..in loops over both own and inherited keys
for (let prop in rabbit) console.log(prop); // jumps, then eats

animal = {
  eats: true,
};

rabbit = {
  jumps: true,
  __proto__: animal,
};

for (let prop in rabbit) {
  let isOwn = rabbit.hasOwnProperty(prop);

  if (isOwn) {
    console.log(`Our: ${prop}`); // Our: jumps
  } else {
    console.log(`Inherited: ${prop}`); // Inherited: eats
  }
}

animal = {
  eats: true,
};

class Rabbit {
  constructor(name) {
    this.name = name;
  }
}

Rabbit.prototype = animal;

rabbit = new Rabbit("White Rabbit"); //  rabbit.__proto__ == animal

alert(rabbit.eats); // true

// Not overwrite Rabbit.prototype totally
// just add to it
Rabbit.prototype.jumps = true;
// the default Rabbit.prototype.constructor is preserved
Rabbit.prototype = {
  jumps: true,
  constructor: Rabbit,
};
