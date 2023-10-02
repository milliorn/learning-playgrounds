const path = require('path'); // Require the path module

const circle = require('./circle.js');
console.log(`The area of a circle of radius 4 is ${circle.area(4)}`);

const Square = require('./square.js');
const mySquare = new Square(2);
console.log(`The area of mySquare is ${mySquare.area()}`);

console.log(__dirname);
console.log(path.dirname(__filename));

console.log(require.main);

const a = require('./a');
a.on('ready', () => {
  console.log('module "a" is ready');
});