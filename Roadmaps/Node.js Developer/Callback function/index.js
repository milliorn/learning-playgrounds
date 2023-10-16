let value = 1;

const doSomething = () => {
  value = 2;
  return value;
};

console.log(value);

doSomething();

console.log(value);