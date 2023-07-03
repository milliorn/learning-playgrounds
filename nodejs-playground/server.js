const express = require("express");
const path = require("path");

// Getting started with Express Web Framework
const app = express();
const PORT = 3000;

// Serving Static Files with Express
app.use("/public", express.static(path.join(__dirname, "static")));

app.get("/", (req, res) => {
  res.sendFile(path.join(__dirname, "static", "index.html"));
});

// Working with Express Http Get Request, Route Params and Query Strings

app.get("/express", (req, res) => {
  res.send("Hello Express");
});

app.get("/example", (req, res) => {
  res.send("example route");
});

app.get("/example/:name/:age", (req, res) => {
  // example
  // http://localhost:3000/example/scott/30
  console.log(req.params);
  // example
  // http://localhost:3000/example/scott/30?tutorial=paramstutorial
  // http://localhost:3000/example/scott/30?tutorial=paramstutorial&sort=byage
  console.log(req.query);
  res.send(`${req.params.name}:${req.params.age}`);
});

app.listen(PORT, () => {
  console.log(`Listening at http://localhost:${PORT}`);
});
