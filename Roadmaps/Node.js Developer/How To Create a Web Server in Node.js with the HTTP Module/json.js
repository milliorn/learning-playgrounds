const http = require('http'); // loading the http module that’s standard with all Node.js installations.

const host = 'localhost'; // define two constants, the host and port that our server will be bound to:
const port = 8000;

const requestListener = (req, res) => {
  res.setHeader("Content-Type", "application/json"); // set the Content-Type header to application/json:
  res.writeHead(200); // 200 is the HTTP status code returned to the client. It traditionally means that everything went OK.
  res.end(`{"message": "This is a JSON response"}`); // send the response body as a JSON string:
}

const server = http.createServer(requestListener); // start the server with the listen function:

server.listen(port, host, () => { // The listen function takes three arguments: the port, the host, and a callback function.
  console.log(`Server is running on http://${host}:${port}`);
});