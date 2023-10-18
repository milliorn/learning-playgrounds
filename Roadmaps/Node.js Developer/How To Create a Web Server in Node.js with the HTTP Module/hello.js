// loading the http module that’s standard with all Node.js installations. 
const http = require('http');

// define two constants, the host and port that our server will be bound to:
const host = 'localhost';
const port = 8000;

// create a server object and pass it a callback function that will be invoked for each connection received:
const requestListener = (req, res) => {
  res.writeHead(200); // 200 is the HTTP status code returned to the client. It traditionally means that everything went OK.
  res.end('Hello, world!\n');
}

const server = http.createServer(requestListener);

// start the server with the listen function:
server.listen(port, host, () => { // The listen function takes three arguments: the port, the host, and a callback function.
  console.log(`Server is running on http://${host}:${port}`);
});