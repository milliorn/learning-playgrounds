// Import the buildSchema function from the GraphQL package
const { buildSchema } = require("graphql");

// Build your GraphQL schema using the buildSchema function
const schema = buildSchema(` 
    type Query {
        users: [User!]!,
        user(id: Int!): User!
    }
    type User {
        id: ID!
        name: String!
        email: String
        posts: [Post!]
    }
    type Post {
        id: ID!
        title: String!
        published: Boolean!
        link: String
        author: User!
    }
`);

module.exports = schema;
