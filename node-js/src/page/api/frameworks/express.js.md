# [Express.js](https://expressjs.com/)

**Express.js** is a minimal and flexible **Node.js web application framework** that provides a robust set of features to build web and mobile applications. It simplifies the process of building APIs and web servers, offering a thin layer of fundamental web application features without obscuring Node.js features.

Here’s a detailed description and example of Express.js, followed by interview questions and answers.

### Key Features of Express.js:

- **Routing**: It provides a powerful routing mechanism that allows developers to map HTTP requests to specific URLs and HTTP methods.
- **Middleware Support**: Express.js allows the use of middleware functions to modify requests and responses, perform logging, handle errors, and more.
- **View System**: It supports various template engines like Pug, EJS, and Handlebars to dynamically generate HTML pages.
- **Minimal Configuration**: Express.js is highly unopinionated and minimal, allowing developers to structure their applications as they see fit.
- **Asynchronous**: Like Node.js, Express.js applications are asynchronous and handle multiple requests efficiently.

### Example of an Express.js Application:

```javascript
const express = require("express");
const app = express();
const port = 3000;

// Middleware to parse incoming JSON requests
app.use(express.json());

// Route to respond with "Hello World!" when a GET request is made to the homepage
app.get("/", (req, res) => {
  res.send("Hello World!");
});

// Route for handling a POST request
app.post("/submit", (req, res) => {
  const data = req.body;
  res.send(`Data received: ${JSON.stringify(data)}`);
});

// Start the server
app.listen(port, () => {
  console.log(`Server is running on http://localhost:${port}`);
});
```

### How It Works:

- **`app.use(express.json())`**: This is middleware that parses incoming JSON requests.
- **Routing**: `app.get()` and `app.post()` define routes for HTTP GET and POST requests.
- **Server**: `app.listen()` starts the server on the specified port.

---

### Express.js Questions and Answers

#### 1. **What is Express.js, and why is it used?**

- **Answer**: Express.js is a web application framework for Node.js. It is used to build web applications and APIs. It simplifies server-side development by providing features like routing, middleware support, and template engines, making it easier to manage HTTP requests and responses.

#### 2. **How do you create a simple server using Express.js?**

- **Answer**:

  ```javascript
  const express = require("express");
  const app = express();

  app.get("/", (req, res) => {
    res.send("Hello World!");
  });

  app.listen(3000, () => {
    console.log("Server is running on port 3000");
  });
  ```

  This creates a simple server that responds with "Hello World!" when accessed at the root URL (`/`).

#### 3. **What are middleware functions in Express.js?**

- **Answer**: Middleware functions are functions that have access to the request object (`req`), response object (`res`), and the next middleware function in the application's request-response cycle. They are used to modify requests and responses, handle errors, perform authentication, logging, and more.

**Example**:

```javascript
app.use((req, res, next) => {
  console.log("Request received at: ", new Date());
  next(); // Pass control to the next handler
});
```

#### 4. **How do you handle different HTTP methods in Express.js?**

- **Answer**: Express provides methods like `app.get()`, `app.post()`, `app.put()`, and `app.delete()` to handle specific HTTP methods. Each method takes a URL path and a callback function to process the request.

**Example**:

```javascript
app.get("/user", (req, res) => {
  res.send("GET request for user");
});

app.post("/user", (req, res) => {
  res.send("POST request for user");
});
```

#### 5. **What is routing in Express.js?**

- **Answer**: Routing refers to how an application responds to client requests at a particular URL endpoint and HTTP method. Express.js provides methods to define routes that handle specific HTTP requests (GET, POST, etc.) for specific URLs.

**Example**:

```javascript
app.get("/home", (req, res) => {
  res.send("Welcome to the home page!");
});
```

#### 6. **How do you handle form data in Express.js?**

- **Answer**: To handle form data, you need middleware like `express.urlencoded()` to parse incoming form data. It allows the server to access the form data as key-value pairs from `req.body`.

**Example**:

```javascript
app.use(express.urlencoded({ extended: true }));

app.post("/form", (req, res) => {
  res.send(`Form Data Received: ${req.body.name}`);
});
```

#### 7. **How do you serve static files in Express.js?**

- **Answer**: Static files (like images, CSS, JavaScript) can be served using the `express.static()` middleware.

**Example**:

```javascript
app.use(express.static("public")); // Serve files from the 'public' directory
```

This makes files in the `public` directory accessible via the browser.

#### 8. **What is the role of `next()` in Express.js?**

- **Answer**: The `next()` function is used to pass control to the next middleware function in the stack. If `next()` is not called, the request-response cycle will be stuck, and the server will not send a response.

#### 9. **How do you handle 404 errors in Express.js?**

- **Answer**: You can handle 404 errors by defining a middleware at the end of your routing stack that sends a 404 response when no other routes match the incoming request.

**Example**:

```javascript
app.use((req, res) => {
  res.status(404).send("Page Not Found");
});
```

#### 10. **What is `express.json()` middleware used for?**

- **Answer**: The `express.json()` middleware is used to parse incoming JSON request bodies into JavaScript objects, which can be accessed via `req.body`.

#### 11. **What are route parameters in Express.js?**

- **Answer**: Route parameters are dynamic segments of a URL that can be captured and used inside the route handler.

**Example**:

```javascript
app.get("/user/:id", (req, res) => {
  res.send(`User ID is ${req.params.id}`);
});
```

#### 12. **How do you redirect in Express.js?**

- **Answer**: You can redirect to another URL using the `res.redirect()` method.

**Example**:

```javascript
app.get("/old-page", (req, res) => {
  res.redirect("/new-page");
});
```

#### 13. **How do you handle errors in Express.js?**

- **Answer**: Error-handling middleware can be defined by passing four arguments `(err, req, res, next)` to the middleware function.

**Example**:

```javascript
app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(500).send("Something went wrong!");
});
```

#### 14. **How do you use template engines with Express.js?**

- **Answer**: Express.js supports various template engines like Pug, EJS, and Handlebars. You need to set the view engine and render the views.

**Example**:

```javascript
app.set("view engine", "pug");

app.get("/home", (req, res) => {
  res.render("home", { title: "Welcome", message: "Hello World" });
});
```

#### 15. **What is the difference between `app.use()` and route handlers like `app.get()`?**

- **Answer**: `app.use()` is used to register middleware that is executed for every incoming request, while route handlers like `app.get()`, `app.post()` are used to handle specific HTTP methods for specific routes.

#### 16. **How can you set custom headers in an Express.js application?**

- **Answer**: Custom headers can be set using the `res.set()` method, which allows you to add or modify response headers.

**Example**:

```javascript
app.get("/custom-header", (req, res) => {
  res.set("X-Custom-Header", "MyCustomHeaderValue");
  res.send("Header has been set!");
});
```

#### 17. **How do you handle file uploads in Express.js?**

- **Answer**: To handle file uploads in Express.js, you can use middleware like `multer` to process multipart/form-data, which is used for file uploads.

**Example** (using `multer`):

```javascript
const multer = require("multer");
const upload = multer({ dest: "uploads/" });

app.post("/upload", upload.single("file"), (req, res) => {
  res.send(`File uploaded: ${req.file.filename}`);
});
```

#### 18. **What is `express.Router()` and how is it used?**

- **Answer**: `express.Router()` is used to create modular, mountable route handlers. It allows you to organize your routes into smaller, separate files or modules and mount them as middleware on a specific path.

**Example**:

```javascript
const express = require("express");
const router = express.Router();

router.get("/users", (req, res) => {
  res.send("List of users");
});

app.use("/api", router); // Mount router at /api
```

### 19. **How do you handle CORS in Express.js?**

- **Answer**: CORS (Cross-Origin Resource Sharing) can be handled using the `cors` middleware, which allows or restricts resources to be requested from another domain.

**Example** (using the `cors` package):

```javascript
const cors = require("cors");
app.use(cors()); // Enable CORS for all routes
```

You can also configure CORS to restrict access to certain domains:

```javascript
const corsOptions = {
  origin: "http://example.com",
};
app.use(cors(corsOptions));
```

### 20. **How can you structure an Express.js application for scalability?**

- **Answer**: To scale an Express.js application, follow a modular approach by separating routes, middleware, controllers, services, and database interactions into different files. Use `express.Router()` for route separation and create a folder structure based on the MVC (Model-View-Controller) pattern.

**Example structure**:

```
/controllers
  - userController.js
/routes
  - userRoutes.js
/models
  - userModel.js
/services
  - userService.js
/app.js
```

### 21. **What is the difference between `app.use()` and `app.all()` in Express.js?**

- **Answer**:
  - **`app.use()`**: It is used to register middleware that applies to every incoming request regardless of HTTP method or path (if no path is specified).
  - **`app.all()`**: It handles all HTTP methods for a specific path. It can be useful when you need to perform operations on any request to a certain route.

**Example**:

```javascript
// app.use() example
app.use((req, res, next) => {
  console.log("Request received");
  next();
});

// app.all() example
app.all("/secret", (req, res, next) => {
  res.send("This route matches all HTTP methods");
});
```

### 22. **How do you handle sessions in an Express.js application?**

- **Answer**: Sessions can be handled using the `express-session` middleware. Sessions allow you to store user information across multiple HTTP requests.

**Example** (using `express-session`):

```javascript
const session = require("express-session");

app.use(
  session({
    secret: "secret_key",
    resave: false,
    saveUninitialized: true,
  }),
);

app.get("/login", (req, res) => {
  req.session.user = { username: "JohnDoe" };
  res.send("User logged in");
});

app.get("/profile", (req, res) => {
  res.send(`Welcome ${req.session.user.username}`);
});
```

### 23. **How do you implement rate limiting in Express.js?**

- **Answer**: Rate limiting can be implemented using the `express-rate-limit` middleware to prevent excessive requests from a single client, which is useful for mitigating DDoS attacks.

**Example**:

```javascript
const rateLimit = require("express-rate-limit");

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100, // Limit each IP to 100 requests per windowMs
});

app.use(limiter); // Apply rate limiting globally
```

### 24. **How do you define a catch-all route in Express.js?**

- **Answer**: A catch-all route is a route that handles any requests that don’t match previous defined routes. This is often used to handle 404 errors.

**Example**:

```javascript
app.use((req, res) => {
  res.status(404).send("Page not found");
});
```

### 25. **How do you debug an Express.js application?**

- **Answer**: Debugging can be done using the built-in `console.log()` method or by using more sophisticated tools like the `debug` package. You can also use Node.js debuggers, such as `node --inspect` or debugging tools like VSCode’s built-in debugger.

**Example** (using `debug`):

```javascript
const debug = require("debug")("app:server");

app.get("/", (req, res) => {
  debug("Handling GET request for /");
  res.send("Hello World");
});
```

### 26. **What is helmet.js, and how is it related to Express.js?**

- **Answer**: `helmet.js` is a middleware that helps secure Express.js applications by setting various HTTP headers. It protects your app from some well-known web vulnerabilities.

**Example**:

```javascript
const helmet = require("helmet");
app.use(helmet()); // Enable security headers
```

### 27. **How do you implement logging in an Express.js application?**

- **Answer**: Logging in Express.js can be implemented using the `morgan` middleware, which provides HTTP request logging.

**Example**:

```javascript
const morgan = require("morgan");
app.use(morgan("combined")); // Log requests in 'combined' format
```

### 28. **How can you test an Express.js API?**

- **Answer**: Express.js APIs can be tested using tools like **Postman** or **cURL**. For automated testing, you can use libraries like **Jest** and **Supertest**.

**Example** (using Jest and Supertest):

```javascript
const request = require("supertest");
const app = require("./app"); // Your express app

test("GET /", async () => {
  const res = await request(app).get("/");
  expect(res.statusCode).toEqual(200);
  expect(res.text).toContain("Hello World");
});
```

### 29. **How do you handle file downloads in Express.js?**

- **Answer**: You can handle file downloads using the `res.download()` method, which prompts the user to download the file.

**Example**:

```javascript
app.get("/download", (req, res) => {
  const file = `${__dirname}/files/report.pdf`;
  res.download(file); // Prompt user to download the file
});
```

### 30. **What is `res.json()` in Express.js, and how does it differ from `res.send()`?**

- **Answer**: `res.json()` is used to send a JSON response, while `res.send()` can send various types of responses (HTML, plain text, JSON, etc.). `res.json()` also ensures proper JSON formatting, whereas `res.send()` doesn't guarantee it.

**Example**:

```javascript
app.get("/data", (req, res) => {
  res.json({ message: "This is JSON" }); // Sends JSON response
});
```

### 31. **What is `res.locals` in Express.js, and how is it used?**

- **Answer**: `res.locals` is an object that contains local variables for a response. It is useful for passing data between middleware and views within the same request-response cycle. It is only available for the duration of that particular request.

**Example**:

```javascript
app.use((req, res, next) => {
  res.locals.user = { name: "John", role: "admin" };
  next();
});

app.get("/", (req, res) => {
  res.send(`Hello, ${res.locals.user.name}`); // Access the data set in res.locals
});
```

### 32. **How can you stream data in an Express.js application?**

- **Answer**: You can stream data in Express.js by using Node.js streams, especially useful for handling large files without loading them into memory all at once. Streaming can be achieved using `res.write()` and `res.end()` for custom streams or piping file streams.

**Example**:

```javascript
const fs = require("fs");

app.get("/stream", (req, res) => {
  const stream = fs.createReadStream("large-file.txt");
  stream.pipe(res); // Pipe the file stream to the response
});
```

### 33. **How do you handle HTTPS in an Express.js application?**

- **Answer**: To handle HTTPS in an Express.js application, you need to use the `https` module from Node.js. You will also need SSL certificates (key and cert) to establish a secure connection.

**Example**:

```javascript
const https = require("https");
const fs = require("fs");
const express = require("express");
const app = express();

const options = {
  key: fs.readFileSync("server.key"),
  cert: fs.readFileSync("server.cert"),
};

https.createServer(options, app).listen(443, () => {
  console.log("Server running on https://localhost");
});

app.get("/", (req, res) => {
  res.send("Secure connection established");
});
```

### 34. **What is `app.param()` in Express.js, and how is it used?**

- **Answer**: `app.param()` is used to define middleware that will be triggered when a specific route parameter is present in the URL. It is useful for pre-loading or validating parameters before route handlers are invoked.

**Example**:

```javascript
app.param("userId", (req, res, next, id) => {
  // Simulate loading a user object from the database
  req.user = { id, name: "John Doe" };
  next();
});

app.get("/users/:userId", (req, res) => {
  res.send(`User ID: ${req.user.id}, Name: ${req.user.name}`);
});
```

### 35. **How do you manage multiple environments (e.g., development, production) in an Express.js app?**

- **Answer**: Managing multiple environments can be done by using environment variables or configuration files. Express automatically sets `process.env.NODE_ENV` to "development" if not specified. You can configure different behaviors based on the environment.

**Example**:

```javascript
if (process.env.NODE_ENV === "production") {
  app.use(compression()); // Enable compression in production
} else {
  app.use(morgan("dev")); // Enable detailed logging in development
}
```

Additionally, using packages like `dotenv` helps load environment variables from a `.env` file.

### 36. **What is middleware in Express.js, and what are the different types?**

- **Answer**: Middleware functions are functions that have access to the request object (`req`), the response object (`res`), and the `next` function in the application’s request-response cycle. They can execute code, make changes to the request or response, end the request-response cycle, or call the next middleware function.
  - **Application-level middleware**: Bound to the app object using `app.use()` or `app.METHOD()`.
  - **Router-level middleware**: Similar to application-level middleware but bound to an instance of `express.Router()`.
  - **Error-handling middleware**: Takes four arguments (err, req, res, next) and is used for handling errors.
  - **Built-in middleware**: Provided by Express, such as `express.static()` for serving static files.

**Example**:

```javascript
// Application-level middleware
app.use((req, res, next) => {
  console.log("Request received");
  next(); // Move to the next middleware/route
});

// Error-handling middleware
app.use((err, req, res, next) => {
  res.status(500).send("Something broke!");
});
```

### 37. **How do you implement content negotiation in Express.js?**

- **Answer**: Content negotiation can be implemented by using the `req.accepts()` method, which checks the `Accept` header of the request and serves the content in the appropriate format based on client preferences.

**Example**:

```javascript
app.get("/data", (req, res) => {
  if (req.accepts("json")) {
    res.json({ message: "Hello, JSON" });
  } else if (req.accepts("html")) {
    res.send("<h1>Hello, HTML</h1>");
  } else {
    res.send("Hello, plain text");
  }
});
```

### 38. **How can you handle errors in an Express.js application?**

- **Answer**: Errors can be handled using error-handling middleware. It is defined with four arguments: `err, req, res, next`. This middleware will catch any errors that occur in the application.

**Example**:

```javascript
app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(500).send("Something broke!");
});
```

You can also handle specific errors in routes by checking for certain conditions and throwing an error or using the `next()` function with an error.

### 39. **What are the benefits of using `express.static()` middleware?**

- **Answer**: The `express.static()` middleware is used to serve static files such as HTML, CSS, JavaScript, images, and more. It simplifies the process of serving these files by specifying a directory from which files can be automatically served by the Express server. It also includes caching mechanisms for improved performance.

**Example**:

```javascript
app.use(express.static("public")); // Serve static files from the 'public' directory

// Files like /public/style.css can be accessed directly at /style.css
```

### 40. **How do you set up a proxy in Express.js?**

- **Answer**: To set up a proxy in Express.js, you can use middleware such as `http-proxy-middleware` to forward requests to another server.

**Example**:

```javascript
const { createProxyMiddleware } = require("http-proxy-middleware");

app.use(
  "/api",
  createProxyMiddleware({
    target: "http://example.com",
    changeOrigin: true,
  }),
);
```

This will proxy all requests made to `/api` to `http://example.com`.

### 41. **What is `res.redirect()` in Express.js?**

- **Answer**: The `res.redirect()` method is used to redirect the client to a different URL. It sends a 302 Found status code by default or a different status code if specified.

**Example**:

```javascript
app.get("/old-route", (req, res) => {
  res.redirect("/new-route"); // Redirect to another route
});

// Optionally, you can specify a status code
res.redirect(301, "/new-route"); // 301 Moved Permanently
```

### 42. **What is the `next()` function in Express.js, and when would you use it?**

- **Answer**: The `next()` function is used to pass control to the next middleware function in the stack. If a middleware function does not call `next()`, the request will be left hanging and not proceed to the next middleware or route handler.

**Example**:

```javascript
app.use((req, res, next) => {
  console.log("This runs for every request");
  next(); // Pass control to the next middleware
});

app.get("/", (req, res) => {
  res.send("Hello, world!");
});
```

### 43. **How do you handle authentication in an Express.js application?**

- **Answer**: Authentication can be handled using various strategies, depending on the method you prefer, such as **JWT (JSON Web Tokens)** for token-based authentication or **OAuth** for third-party authentication services. Middleware like `passport.js` is often used for implementing authentication.

**Example** (using Passport for local strategy):

```javascript
const passport = require("passport");
const LocalStrategy = require("passport-local").Strategy;

passport.use(
  new LocalStrategy((username, password, done) => {
    // Validate the user here
    // Call done(null, user) if successful, or done(null, false) if not
  }),
);

app.post(
  "/login",
  passport.authenticate("local", {
    successRedirect: "/dashboard",
    failureRedirect: "/login",
  }),
);
```
