const EventEmitter = require("events");
const fs = require("fs");
const http = require("http");
const tutorial = require("./tutorial");
const zlib = require("zlib");
const { Person } = require("./Person");
const { readLine } = require("./readLine");

console.log(tutorial.sum(1, 2));
console.log(tutorial.PI);

// The Events Module and EventEmitter Class
const eventEmitter = new EventEmitter();

eventEmitter.on("tutorial", (num1, num2) => console.log(num1 + num2));

eventEmitter.emit("tutorial", 2, 3);

let scott = new Person("Scott");
let phil = new Person("Phil");

scott.on("name", () => console.log(`My name is ${scott.name}`));
phil.on("name", () => console.log(`My name is ${phil.name}`));

scott.emit("name");
phil.emit("name");

let num1 = Math.floor(Math.random() * 10 + 1);
let num2 = Math.floor(Math.random() * 10 + 1);
let answer = num1 + num2;

// Working With The ReadLine Module.
readLine.question(`What is ${num1} + ${num2}?\n`, (userInput) => {
  if (userInput.trim() == answer) {
    readLine.close();
  } else {
    readLine.setPrompt("Wrong!\n");
    readLine.prompt();
    readLine.on("line", (userInput) => {
      if (userInput.trim() == answer) {
        readLine.close();
      } else {
        readLine.setPrompt(`Your answer is ${userInput} is wrong!\n`);
        readLine.prompt();
      }
    });
  }
});

readLine.on("close", () => console.log("Correct"));

// Working With File System Module (Creating,Reading,Deleting,Renaming) Files
fs.writeFile("fsExample.txt", "This is filesystem example text.", (err) => {
  if (err) {
    console.error(err);
  } else {
    console.log("File created");
    fs.readFile("fsExample.txt", "utf-8", (err, file) => {
      if (err) {
        console.error(err);
      } else {
        console.log(file);
      }
    });
  }
});

fs.appendFile("fsExample.txt", "\nThis is appended text.", (err) => {
  if (err) {
    console.error(err);
  } else {
    console.log("appended successful.");
  }
});

// Working With File System Module. Creating and Deleting Folders
fs.mkdir("fsDir", (err) => {
  if (err) {
    console.error(err);
  } else {
    console.log("folder created");
  }
});

fs.rmdir("fsDir", (err) => {
  if (err) {
    console.error(err);
  } else {
    console.log("folder deleted");
  }
});

// Working with Readable and Writable Streams
fs.createReadStream("./fsExample.txt");

const readStream = fs.createReadStream("./fsExample.txt", "utf-8");
const writeStream = fs.createWriteStream("./fsExample2.txt");

readStream.on("data", (chunk) => {
  writeStream.write(chunk);
  console.log(chunk);
});

// Creating a Http Server using the Http Module
/*
const server = http.createServer((req, res) => {
  if (req.url === "/home") {
    res.write("Hello Home!");
  } else {
    res.write("Hello World?");
  }
  res.end();
});

server.listen("3000");
*/

// Serving Static Files with Http and File System Module
http
  .createServer((req, res) => {
    /*
    const readStream = fs.createReadStream("./public/index.html");
    res.writeHead(200, { "Content-type": "text/html" });
    readStream.pipe(res);
    */

    const jsonFile = fs.createReadStream("./public/example.json");
    res.writeHead(200, { "Content-type": "application/json" });
    jsonFile.pipe(res);

    /*
    const picture = fs.createReadStream("./public/example.png");
    res.writeHead(200, { "Content-type": "image/png" });
    picture.pipe(res);
    */
  })
  .listen(3000);
