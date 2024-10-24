# [`ejs`](https://ejs.co/)

### EJS (Embedded JavaScript) Template Engine

**EJS** (Embedded JavaScript) is one of the most popular template engines in Node.js. It allows you to generate dynamic HTML with JavaScript and provides features like data binding, conditionals, loops, and includes to make it easier to build reusable templates.

---

### How to Install EJS

To install EJS in a Node.js project, use the following command:

```bash
npm install ejs
```

### Setting Up EJS with Express.js

Here’s an example of how to set up and use EJS in an Express.js project.

#### Step 1: Install Dependencies

```bash
npm install express ejs
```

#### Step 2: Create an Express.js Server

```javascript
// app.js

const express = require("express");
const app = express();

// Set the view engine to EJS
app.set("view engine", "ejs");

// Define a route that renders an EJS template
app.get("/", (req, res) => {
  const data = { name: "John Doe", age: 28 };
  res.render("index", data); // Render 'index.ejs' template with data
});

// Start the server
app.listen(3000, () => {
  console.log("Server running at http://localhost:3000");
});
```

#### Step 3: Create an EJS Template

In the `views` directory, create a file called `index.ejs`:

```html
<!doctype html>
<html>
  <head>
    <title>Welcome Page</title>
  </head>
  <body>
    <h1>Hello, <%= name %>!</h1>
    <p>You are <%= age %> years old.</p>
  </body>
</html>
```

- `<%= name %>` and `<%= age %>` are EJS placeholders used to dynamically inject values into the HTML.

#### Step 4: Run the Server

Start the Express.js server by running:

```bash
node app.js
```

Navigate to `http://localhost:3000`, and the page will display:

```
Hello, John Doe!
You are 28 years old.
```

This is how EJS dynamically generates HTML by injecting the provided data into the template.

---

### Key Features of EJS

1. **Data Binding**: You can pass variables and objects to the EJS template, which will be rendered dynamically in the final HTML.
2. **Conditionals and Loops**: EJS supports JavaScript syntax for conditionals and loops inside templates.
3. **Template Reusability**: You can include and reuse templates for headers, footers, and other components.
4. **Plain HTML with Embedded JS**: EJS is simply HTML with JavaScript embedded, so it's easy for developers who already know HTML and JavaScript.

---

### Example with Conditional Logic and Loop in EJS

```html
<!doctype html>
<html>
  <head>
    <title>User Details</title>
  </head>
  <body>
    <h1>Welcome, <%= user.name %></h1>

    <p>You have <%= user.messages.length %> new messages:</p>
    <ul>
      <% if (user.messages.length > 0) { %> <%
      user.messages.forEach(function(message) { %>
      <li><%= message %></li>
      <% }) %> <% } else { %>
      <p>No new messages.</p>
      <% } %>
    </ul>
  </body>
</html>
```

In this example:

- The loop (`forEach`) iterates over the user's messages and displays each in a `<li>` tag.
- The conditional statement checks whether the user has any messages, and if not, a fallback message is displayed.

---

### Questions and Answers on EJS

#### Q1: **What is EJS and why is it used?**

**Answer**: EJS (Embedded JavaScript) is a simple templating language used in Node.js to generate dynamic HTML pages. It allows you to embed JavaScript code in HTML templates to produce final HTML, which can be rendered on the client side. EJS is primarily used to separate presentation logic from business logic in web applications.

---

#### Q2: **How do you set up EJS with Express.js?**

**Answer**:

1. Install EJS with `npm install ejs`.
2. Set `ejs` as the view engine in Express by using `app.set('view engine', 'ejs')`.
3. Use `res.render('templateName', data)` to render an EJS template with data.

Example:

```javascript
app.set("view engine", "ejs");
app.get("/", (req, res) => {
  res.render("index", { name: "John" });
});
```

---

#### Q3: **How do you pass variables from the server to an EJS template?**

**Answer**: Variables are passed as an object to the `res.render()` method. In the template, you can use `<%= variable %>` to output the value.

Example:

```javascript
app.get("/", (req, res) => {
  const user = { name: "John", age: 30 };
  res.render("index", user);
});
```

---

#### Q4: **What is the difference between `<%= %>` and `<% %>` in EJS?**

**Answer**:

- `<%= %>` is used to output (escape) the value of a variable or expression directly into the HTML.
- `<% %>` is used for JavaScript logic (like loops and conditionals) that doesn’t output anything directly.

---

#### Q5: **How can you include one EJS template inside another?**

**Answer**: You can use the `<%- include('templateName') %>` syntax to include one EJS file inside another.

Example:

```html
<!doctype html>
<html>
  <body>
    <%- include('header') %>
    <h1>Welcome to my website!</h1>
    <%- include('footer') %>
  </body>
</html>
```

---

#### Q6: **What is the purpose of the `app.set('view engine', 'ejs')` statement in Express?**

**Answer**: This line tells the Express framework to use the EJS template engine to render views (HTML pages). It sets EJS as the default view engine for the app, so you don't need to specify the file extension when rendering views.

---

#### Q7: **Can EJS be used for both server-side and client-side templating?**

**Answer**: Yes, EJS can be used both server-side (with Node.js and Express) and client-side (directly in the browser) for rendering dynamic HTML. However, server-side usage is more common in Node.js applications.

---

#### Q8: **What is the difference between `res.render()` and `res.send()` in Express when using EJS?**

**Answer**:

- `res.render()`: Used to render a view template (like EJS) and send the resulting HTML to the client.
- `res.send()`: Directly sends a response to the client without rendering any template.

---

#### Q9: **How can you use loops in EJS to render a list of items dynamically?**

**Answer**: You can use JavaScript loops like `forEach` inside EJS templates.

Example:

```html
<ul>
  <% items.forEach(function(item) { %>
  <li><%= item %></li>
  <% }) %>
</ul>
```

---

#### Q10: **How do you handle conditionals in EJS?**

**Answer**: You can use standard JavaScript conditionals inside EJS templates using `<% if (condition) { %> ... <% } %>`.

Example:

```html
<% if (user.loggedIn) { %>
<p>Welcome, <%= user.name %>!</p>
<% } else { %>
<p>Please log in.</p>
<% } %>
```

---

### More Questions and Answers on EJS

#### Q11: **How can you escape HTML content in EJS to prevent XSS attacks?**

**Answer**: By default, EJS escapes content when using `<%= %>`. To output unescaped content, you would use `<%- %>`. To prevent XSS attacks, make sure that user input is sanitized, and `<%- %>` is only used when you trust the content.

---

#### Q12: **How does EJS handle partial templates or partial views?**

**Answer**: EJS allows partial templates through the `include` directive. This can be used to include headers, footers, and other reusable components across multiple pages.

Example:

```html
<%- include('header') %>
<h1>Welcome to my website!</h1>
<%- include('footer') %>
```

---

#### Q13: **How can you pass multiple variables to an EJS template?**

**Answer**: You pass multiple variables by creating an object and passing it to the `res.render()` function.

Example:

```javascript
app.get("/", (req, res) => {
  res.render("index", { name: "John", age: 25, city: "New York" });
});
```

In the EJS template:

```html
<p>Name: <%= name %></p>
<p>Age: <%= age %></p>
<p>City: <%= city %></p>
```
