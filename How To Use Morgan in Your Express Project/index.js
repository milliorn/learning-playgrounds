const bodyParser = require('body-parser');
const express = require('express');
const morgan = require('morgan');

const app = express();  // express app
const port = 3000;  // port to listen on
const host = 'http://localhost';  // host to listen on

app.use(bodyParser.urlencoded({ extended: true })); // Middleware to parse URL-encoded bodies


morgan.token('host', function (req, res) {
  console.log(req.hostname);
  return req.hostname;
});

// Use the 'param' token with the 'id' parameter
morgan.token('param', (req, res, param) => req.params[ param ]);

app.use(morgan('tiny'));
app.use(morgan(':method :host :status :param[id] :res[content-length] - :response-time ms'));

app.get('/', (req, res) => {
  res.send('Hello World!');
});

// Create a route that accepts an 'id' parameter
app.get('/example/:id', (req, res) => {
  // Access the 'id' parameter
  const id = req.params.id;

  // Return a response
  res.send(`Received 'id' parameter: ${id}`);
});

app.listen(port, () => {
  console.log(`Server running at ${host}:${port}`);
});
