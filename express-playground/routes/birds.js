// Use the express.Router class to create modular, mountable route handlers. A Router instance is a complete middleware and routing system; for this reason, it is often referred to as a “mini-app”.

// The following example creates a router as a module, loads a middleware function in it, defines some routes, and mounts the router module on a path in the main app.

const express = require("express");
const router = express.Router();

// middleware
router.use((req, res, next) => {
  console.log("Time: ", Date.now());
  next();
});

// home page route
router.get("/", (req, res) => {
  res.send("Birds home page");
});

// about page route
router.get("/about", (req, res) => {
  res.send("About birds");
});

module.exports = router;
