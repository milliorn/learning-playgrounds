require("dotenv").config();

// Import the express framework, which helps us build web applications
const express = require("express");

// Destructure the graphqlHTTP function from the express-graphql package
const { graphqlHTTP } = require("express-graphql");

// Import the GraphQL schema you defined in the schema.js file
const schema = require("./schemas/schema.js");

// Import the resolver functions that provide the logic for your queries and mutations
const resolvers = require("./resolvers/resolver.js");

// Create an instance of the Express application
const app = express();

// Set up a middleware to handle GraphQL requests
app.use(
  "/graphql",
  graphqlHTTP({
    // Provide the schema you defined to the graphqlHTTP middleware
    schema,
    // Provide the resolver functions to handle the actual data retrieval and manipulation
    rootValue: resolvers,
    // Enable the GraphiQL interface for easy testing and exploration
    graphiql: true
  })
);

// Define the port number on which the server will listen
const port = process.env.PORT || 3001;

// Start the server and listen for incoming requests on the specified port
app.listen(port, () => {
  console.log(`Server ready at http://localhost:${port}/graphql`);
});
