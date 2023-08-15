// Import the Users data from the dummy-data folder
const { Users } = require("../dummy-data/data.js");

// Define resolver functions for your GraphQL schema
const resolvers = {
  // Resolver for the 'users' query
  users: async () => {
    // Return the Users data when the 'users' query is executed
    return Users;
  },
  // Resolver for the 'user' query
  user: async ({ id }, context) => {
    // Find and return a user based on the provided 'id' argument
    return Users.find(user => user.id === id);
  }
};

// Export the resolver functions to be used in your schema
module.exports = resolvers;
