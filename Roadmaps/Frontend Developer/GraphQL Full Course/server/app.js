// Import necessary modules
require('dotenv').config();
const express = require('express');
const { createHandler } = require('graphql-http/lib/use/express');
const schema = require('./schemas/schema.js');

// Create an instance of the Express application
const app = express();

// Set up GraphQL endpoint using createHandler function
app.all('/graphql', createHandler({ schema }));

const port = process.env.PORT || 3001;

// Start the server
app.listen(port, () => {
  console.log(`Server is running on http://localhost:${port}`);
});
