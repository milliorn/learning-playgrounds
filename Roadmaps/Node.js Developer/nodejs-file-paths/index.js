const path = require('path');

const notes = '/users/joe/notes.txt';

console.log(path.dirname(notes)); // /users/joe
console.log(path.basename(notes)); // notes.txt
console.log(path.extname(notes)); // .txt

console.log(path.basename(notes, path.extname(notes))); // notes

const name = 'joe';
const newName = path.join('/', 'users', name, 'notes.txt'); // '/users/joe/notes.txt'
console.log(newName);

console.log(path.resolve('notes.txt')); // /Users/joe/notes.txt
console.log(path.resolve('joe.txt')); // /Users/joe/joe.txt
console.log(path.resolve('/etc', 'joe.txt')); // /etc/joe.txt