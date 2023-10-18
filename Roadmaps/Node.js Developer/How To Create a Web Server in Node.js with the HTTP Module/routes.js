const http = require('http'); // loading the http module that’s standard with all Node.js installations.

const host = 'localhost'; // define two constants, the host and port that our server will be bound to:
const port = 8000;

const books = JSON.stringify([
  { title: "The Alchemist", author: "Paulo Coelho", year: 1988 },
  { title: "The Prophet", author: "Kahlil Gibran", year: 1923 }
]);

const authors = JSON.stringify([
  { name: "Paulo Coelho", countryOfBirth: "Brazil", yearOfBirth: 1947 },
  { name: "Kahlil Gibran", countryOfBirth: "Lebanon", yearOfBirth: 1883 }
]);

const requestListener = (req, res) => {
  res.setHeader("Content-Type", "application/json");
  switch (req.url) {
    case "/books":
      res.writeHead(200);
      res.end(books);
      break;
    case "/authors":
      res.writeHead(200);
      res.end(authors);
      break;
    default:
      res.writeHead(404);
      res.end(JSON.stringify({ error: "Resource not found" }));
  }
}

const server = http.createServer(requestListener); // start the server with the listen function:

server.listen(port, host, () => { // The listen function takes three arguments: the port, the host, and a callback function.
  console.log(`Server is running on http://${host}:${port}`);
});