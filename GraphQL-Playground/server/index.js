require("dotenv").config();

const colors = require("colors");
const connectDB = require("./config/db");

const express = require("express");
const app = express();
const { graphQlHTTp, graphqlHTTP } = require("express-graphql");
const schema = require("./schema/schema");
const cors = require("cors");

const port = process.env.PORT || 8000;

/** connect database */
connectDB();

/** fixes issue with fetch from origin has been blocked by CORS policy */
app.use(cors());

/** create endpoint for GraphQL in development mode */
app.use(
  "/graphql",
  graphqlHTTP({
    schema,
    graphiql: process.env.NODE_ENV === "development",
  })
);

/** log results */
app.listen(port, () => {
  console.log(`Server running on http://localhost:${port}`);
  console.log(`GraphQL running on http://localhost:${port}/graphql`);
});
