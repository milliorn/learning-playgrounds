// https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Asynchronous/Introducing#synchronous_programming
let name = "Miriam";
let greeting = `Hello, my name is ${name}!`;
console.log(greeting);
// "Hello, my name is Miriam!"
function makeGreeting(name) {
  return `Hello, my name is ${name}!`;
}
name = "Miriam";
greeting = makeGreeting(name);
console.log(greeting);
