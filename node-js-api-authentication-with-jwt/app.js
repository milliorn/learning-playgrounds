const express = require("express");
const jwt = require("jsonwebtoken");
const port = process.env.PORT || 5000;
// initialize express
const app = express();

// mock user
const user = {
  id: 1,
  username: "Scott",
  email: "scott@gmail.com",
};

app.get("/api", (req, res) => {
  res.json({ message: "Welcome to API" });
});

app.post("/api/posts", verifyToken, (req, res) => {
  jwt.verify(req.token, "secretkey", (err, authData) => {
    if (err) {
      res.sendStatus(403);
    } else {
      res.json({ message: "Post created...", authData });
    }
  });

  res.json({ message: `Post created at ${new Date().toLocaleTimeString()}` });
});

// implement json web token with a route
app.post("/api/login", (req, res) => {
  jwt.sign({ user: user }, "secretkey", { expiresIn: "60s" }, (err, token) => {
    res.json({ token: token });
  });
});

// Token format
// authorization: bearer <access_token>

// verify token
function verifyToken(req, res, next) {
  // Get auth header value. When we send our token we want to send it in header as auth value.
  const bearerHeader = req.headers["authorization"];

  // check if bearer is undefined
  if (typeof bearerHeader !== "undefined") {
    // split token at the space
    const bearer = bearerHeader.split(" ");

    // get token from array that we just split
    const bearerToken = bearer[1];

    // set the token
    req.token = bearerToken;

    // call next middleware
    next();
  } else {
    // forbidden
    res.sendStatus(403);
  }
}

app.listen(5000, () => {
  console.log(`Listening at http://localhost:${port}`);
});
