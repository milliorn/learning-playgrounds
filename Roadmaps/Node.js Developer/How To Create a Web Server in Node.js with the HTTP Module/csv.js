const http = require('http'); // loading the http module that’s standard with all Node.js installations.

const host = 'localhost'; // define two constants, the host and port that our server will be bound to:
const port = 8000;

const requestListener = function (req, res) {
  res.setHeader("Content-Type", "text/csv");
  res.setHeader("Content-Disposition", "attachment;filename=oceanpals.csv");
  res.writeHead(200);
  res.end(`id,name,email\n1,Sammy Shark,shark@ocean.com`);
};

const server = http.createServer(requestListener); // start the server with the listen function:

server.listen(port, host, () => { // The listen function takes three arguments: the port, the host, and a callback function.
  console.log(`Server is running on http://${host}:${port}`);
});