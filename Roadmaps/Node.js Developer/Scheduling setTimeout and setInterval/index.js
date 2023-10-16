function sayHello() {
  console.log('Hello');
}

setTimeout(sayHello, 1000);

function sayHi(phrase, who) {
  console.log(phrase + ', ' + who);
}

setTimeout(sayHi, 1000, "Hello", "John"); // Hello, John

let timerId = setTimeout(() => console.log("never happens"), 1000);
console.log(timerId); // timer identifier

clearTimeout(timerId);
console.log(timerId); // same identifier (doesn't become null after canceling)

// repeat with the interval of 2 seconds
timerId = setInterval(() => console.log('tick'), 2000);

// after 5 seconds stop
setTimeout(() => { clearInterval(timerId); console.log('stop'); }, 5000);