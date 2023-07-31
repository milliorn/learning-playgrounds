class Person {
  constructor(name) {
    this.name = name;
  }
  getName() {
    return this.name;
  }
}

let john = new Person("John Doe");

console.log(john.getName());
console.log(john instanceof Person); // true
console.log(john instanceof Object); // true
