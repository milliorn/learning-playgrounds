require('dotenv').config() // dotenv

const express = require('express'); // import express
const app = express(); // create express app
const port = process.env.PORT || 3001; // port

// console.log(process.env)

app.listen(port, () => {
  console.log(`Server is running on http://localhost:${port}`);
}); // listen for requests