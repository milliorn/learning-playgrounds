const person = {
  name: "Jon",
  walk() {},
  talk() {},
};

person.talk();

const targetMember = "name";
person[targetMember.value] = "Jack";
