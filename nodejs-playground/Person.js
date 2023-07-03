const EventEmitter = require("events");

class Person extends EventEmitter {
  constructor(name) {
    super();
    this._name = name;
  }

  get name() {
    return this._name;
  }
}
exports.Person = Person;
