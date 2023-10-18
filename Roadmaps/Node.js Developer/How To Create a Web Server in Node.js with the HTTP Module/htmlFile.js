const http = require('http'); // loading the http module that’s standard with all Node.js installations.
const fs = require('fs').promises; // loading the fs module that’s standard with all Node.js installations.
const host = 'localhost'; // define two constants, the host and port that our server will be bound to:
const port = 8000;

const requestListener = (req, res) => {
  fs.readFile(__dirname + "/index.html") // read the file index.html
    .then(contents => { // then, when the file is read, set the content type and write a 200 response with the contents of the file.
      res.setHeader("Content-Type", "text/html");
      res.writeHead(200);
      res.end(contents);
    })
    .catch(err => { // if there is an error, set the content type and write a 500 response with the error message.
      res.writeHead(500);
      res.end(err);
      return;
    });
}

const server = http.createServer(requestListener); // start the server with the listen function:

server.listen(port, host, () => { // The listen function takes three arguments: the port, the host, and a callback function.
  console.log(`Server is running on http://${host}:${port}`);
});