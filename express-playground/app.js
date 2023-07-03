const birds = require("./routes/birds");

const express = require("express");
const cookieParser = require("cookie-parser");
const cookieValidator = require("./cookieValidator");

const jwt = require("jsonwebtoken");

const app = express();
const port = process.env.PORT || 5000;

// jwt authentication

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

// end jwt authentication

// Pre-caching
// https://www.freecodecamp.org/news/a-detailed-guide-to-pre-caching/
const nodecache = require("node-cache");
require("isomorphic-fetch");

//Creating the node-cache instance
const cache = new nodecache({ stdTTL: 10 });

//We are using the fake API available at <https://jsonplaceholder.typicode.com/>
const baseURL = "https://jsonplaceholder.typicode.com/posts/";

//Pre-caching Popular Posts
[1, 2, 3].map(async (id) => {
  const fakeAPIURL = baseURL + id;
  const data = await fetch(fakeAPIURL).then((response) => response.json());
  cache.set(id, data);
  console.log(`Post Id ${id} cached`);
});

//API Endpoint to demonstrate caching
app.get("/posts/:id", async (req, res) => {
  const id = req.params.id;
  if (cache.has(id)) {
    console.log("Fetching data from the Node Cache");
    res.send(cache.get(id));
  } else {
    const fakeAPIURL = baseURL + id;
    const data = await fetch(fakeAPIURL).then((response) => response.json());

    cache.set(req.params.id, data);
    console.log("Fetching Data from the API");
    res.send(data);
  }
});
//  End Pre-caching

// If you run the express app from another directory, it’s safer to use the absolute path of the directory that you want to serve:
const path = require("path");

// Respond with Hello World! on the homepage:
app.get("/", (req, res) => {
  res.send("Hello World!");
});

// Respond with request header with for host to be view/use in postman, etc.
app.get("/header", (req, res) => {
  res.send(req.header("host"));
});

// Same as above for header but for user-agent
app.get("/useragent", (req, res) => {
  res.send(req.header("user-agent"));
});

// Same as above but instead get the raw headers
app.get("/rawheaders", (req, res) => {
  res.send(req.rawHeaders);
});

// send data to the server in a request body
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.post("/sendrequest", (req, res) => {
  res.send(req.body);
});

// send data to the server about content type
app.post("/contenttype", (req, res) => {
  res.send(req.header("Content-Type"));
});

app.post("/login", (req, res) => {
  if (!req.header("x-auth-token")) {
    return res.status(400).send("No token");
  } else if (req.header("x-auth-token") !== "123456") {
    return res.status(401).send("Not authorized");
  } else {
    res.send("Logged in");
  }
});

// This route path will match requests to /about.
app.get("/about", (req, res) => {
  res.send("about");
});

// This route path will match requests to /random.text.
app.get("/random.text", (req, res) => {
  res.send("random.text");
});

// This route path will match acd and abcd.
app.get("/ab?cd", (req, res) => {
  res.send("ab?cd");
});

// This route path will match abcd, abbcd, abbbcd, and so on.
app.get("/ab+cd", (req, res) => {
  res.send("ab+cd");
});

// This route path will match abcd, abxcd, abRANDOMcd, ab123cd, and so on.
app.get("/ab*cd", (req, res) => {
  res.send("ab*cd");
});

// This route path will match /abe and /abcde.
app.get("/ab(cd)?e", (req, res) => {
  res.send("ab(cd)?e");
});

// This route path will match anything with an "z" in it.
app.get(/z/, (req, res) => {
  res.send("/z/");
});

// This route path will match butterfly and dragonfly, but not butterflyman, dragonflyman, and so on.
app.get(/.*fly$/, (req, res) => {
  res.send("./*fly$/");
});

// To define routes with route parameters, simply specify the route parameters in the path of the route as shown below.
app.get("/user/:userId/books/:bookId", (req, res) => {
  res.send(req.params);
});

// A single callback function can handle a route.
app.get("/example/a", (req, res) => {
  res.send("Hello A!");
});

// More than one callback function can handle a route (make sure you specify the next object).
app.get(
  "/example/b",
  (req, res, next) => {
    console.log("Response will be sent by next()");
    next();
  },
  (req, res) => {
    res.send("Hello from B!");
  }
);

// An array of callback functions can handle a route.
const callbackFirst = (req, res, next) => {
  console.log("callback first");
  next();
};

const callbackSecond = (req, res, next) => {
  console.log("callback second");
  next();
};

const callbackThird = (req, res, next) => {
  console.log("callback third");
  next();
};

// A combination of independent functions and arrays of functions can handle a route.
app.get(
  "/example/callback",
  [callbackFirst, callbackSecond, callbackThird],
  (req, res, next) => {
    console.log("response sent by next function");
    next();
  },
  (req, res) => {
    res.send("Hello callbacks!");
  }
);

// You can create chainable route handlers for a route path by using app.route(). Because the path is specified at a single location, creating modular routes is helpful, as is reducing redundancy and typos.
app
  .route("/book")
  .get((req, res) => {
    res.send("Get a random book");
  })
  .post((req, res) => {
    res.send("Add a book");
  })
  .put((req, res) => {
    res.send("Update the book");
  });

// Middleware function requestTime
const requestTime = (req, res, next) => {
  req.requestTime = Date.now();
  next();
};

app.use(requestTime);

app.get("/time", (req, res) => {
  let responseText = `Hello World!<br>`;
  responseText += `<small>Requested at: ${req.requestTime}</small>`;
  res.send(responseText);
});

// Here we use the cookie-parser middleware to parse incoming cookies off the req object and pass them to our cookieValidator function. The validateCookies middleware returns a Promise that upon rejection will automatically trigger our error handler.
async function validateCookies(req, res, next) {
  // you must make the function cookieValidator for this to work.
  // await cookieValidator(req.cookies);
}

app.use(cookieParser());
app.use(validateCookies);

app.use((err, req, res, next) => {
  res.status(400).send(err.message);
});

// Create a router file named birds.js in the app directory
// Then, load the router module in the app:
app.use("/birds", birds);

// This shows a middleware function mounted on the /user/:id path. The function is executed for any type of HTTP request on the /user/:id path.
app.use("/user/:id", (req, res, next) => {
  console.log("Request type: ", req.method);
  next();
});

// This shows a route and its handler function (middleware system). The function handles GET requests to the /user/:id path.
app.get("/user/:id", (req, res, next) => {
  res.send("USER");
});

// Use the following code to serve images, CSS files, and JavaScript files in a directory named public

// http://localhost:3000/static/hello.html
app.use("/static", express.static(path.join(__dirname, "public")));

// Respond to POST request on the root route (/), the application’s home page:
app.post("/", (req, res) => {
  res.send("Received POST request");
});

// Respond to a PUT request to the /user route:
app.put("/", (req, res) => {
  res.send("Received a PUT at /user");
});

// Respond to a DELETE request to the /user route:
app.delete("/user", (req, res) => {
  res.send("Received a DELETE request at /user");
});

// There is a special routing method, app.all(), used to load middleware functions at a path for all HTTP request methods. The following handler is executed for requests to the route “/secret” whether using GET, POST, PUT, DELETE, or any other HTTP request method supported in the http module.
app.all("/secret", (req, res) => {
  console.log(`Accessing secret endpoint`);
});

app.listen(port, () => {
  console.log(`Listening at http://localhost:${port}`);
});
