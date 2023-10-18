const http = require('http'); // loading the http module that’s standard with all Node.js installations.

const host = 'localhost'; // define two constants, the host and port that our server will be bound to:
const port = 8000;

const requestListener = (req, res) => {
  res.setHeader("Content-Type", "text/html");
  res.writeHead(200);
  res.end(`<html><body><h1>This is HTML</h1></body></html>`);
}

const server = http.createServer(requestListener); // start the server with the listen function:

server.listen(port, host, () => { // The listen function takes three arguments: the port, the host, and a callback function.
  console.log(`Server is running on http://${host}:${port}`);
});