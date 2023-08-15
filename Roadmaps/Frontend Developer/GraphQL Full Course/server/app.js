// Import necessary modules
require('dotenv').config();
const express = require('express');
const { createHandler } = require('graphql-http/lib/use/express');
const { buildSchema } = require('graphql'); // Import the necessary function to build the schema

// Create an instance of the Express application
const app = express();

// Define your GraphQL schema
const schema = buildSchema(`
  type Query {
    hello: String
  }
`);

// Set up GraphQL endpoint using createHandler function
app.use('/graphql', createHandler({ schema }));

const port = process.env.PORT || 3001;

// Start the server
app.listen(port, () => {
  console.log(`Server is running on http://localhost:${port}`);
});
