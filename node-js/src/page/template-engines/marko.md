# [`marko`](https://markojs.com/)

**Marko** is a popular open-source JavaScript framework developed by eBay for building fast, server-side rendered web applications. Marko focuses on performance and simplicity, providing an efficient way to render HTML on both the server and client. It is often compared with other templating engines like React and Vue, but it differentiates itself with its fine-grained reactivity and ability to do both client and server rendering seamlessly.

---

### Key Features of Marko:

1. **Component-based architecture**: Marko is built around components, which makes UI development modular and reusable.
2. **Server-side rendering (SSR)**: Marko renders content on the server, which leads to faster initial page loads.
3. **Progressive rehydration**: Marko can progressively send JavaScript to the browser, allowing for faster interactive pages without loading unnecessary resources upfront.
4. **Lightweight and performant**: Marko is designed to be lightweight and deliver the best performance for high-traffic websites, making it ideal for applications like e-commerce platforms.
5. **HTML-first approach**: Marko templates look very much like HTML, making it easy to learn and adopt.

---

### Installation

To use Marko in a Node.js application, you can install it via npm:

```bash
npm install marko
```

---

### Example: Basic Usage with Express.js

#### Step 1: Install Dependencies

```bash
npm install express marko @marko/express
```

#### Step 2: Setup Marko in Express.js

```javascript
// app.js

const express = require("express");
const markoExpress = require("@marko/express");
const app = express();

// Register Marko as the view engine
app.use(markoExpress());

// Define a route
app.get("/", (req, res) => {
  res.marko(require("./views/index.marko"), { name: "World" });
});

// Start the server
app.listen(3000, () => {
  console.log("Server is running on http://localhost:3000");
});
```

#### Step 3: Create a Marko Template

In the `views` folder, create a file named `index.marko`:

```marko
<!doctype html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Hello Marko</title>
</head>
<body>
    <h1>Hello ${input.name}!</h1>
</body>
</html>
```

#### Step 4: Run the Server

Run your Express.js server:

```bash
node app.js
```

Navigate to `http://localhost:3000`, and you will see "Hello World!" displayed on the page.

---

### Components in Marko

Marko has a powerful component model that allows you to create reusable UI pieces. Components can be made interactive by attaching JavaScript behaviors to them, and they support both client and server-side rendering.

#### Example: Creating a Marko Component

```marko
// my-component.marko

class {
  onCreate() {
    this.state = { count: 0 };
  }

  increment() {
    this.state.count++;
  }
}

<div>
  <h1>Count: ${state.count}</h1>
  <button onClick('increment')>Increment</button>
</div>
```

In this example, we define a simple counter component that maintains a count state and has a button to increment the count. This is a dynamic, reactive component.

---

### Questions and Answers on Marko

#### Q1: **What is Marko, and how does it compare to other frameworks like React or Vue?**

**Answer**: Marko is a JavaScript framework focused on server-side rendering and performance, designed to make building fast, interactive web applications easier. It differs from frameworks like React or Vue by providing out-of-the-box server-side rendering and progressive rehydration, allowing applications to load faster and ship less JavaScript. While React and Vue are mainly client-side frameworks, Marko prioritizes server-rendered content.

---

#### Q2: **How do you install Marko in a Node.js project?**

**Answer**: Marko can be installed using npm:

```bash
npm install marko
```

Once installed, it can be used as a view engine in Express or any other Node.js framework.

---

#### Q3: **How does Marko handle server-side rendering (SSR)?**

**Answer**: Marko is designed with SSR in mind. It renders components on the server as HTML and sends that HTML to the client. Once on the client, Marko rehydrates the page progressively, making it interactive without having to download all the JavaScript upfront. This leads to better performance compared to traditional client-side rendering.

---

#### Q4: **What is progressive rehydration in Marko?**

**Answer**: Progressive rehydration is a technique where only the JavaScript necessary to make the page interactive is sent to the browser. Marko allows parts of the page to be progressively enhanced with JavaScript as needed, rather than requiring the entire JavaScript payload to be loaded before the page becomes interactive.

---

#### Q5: **How does Marko differ in terms of syntax compared to React or Vue?**

**Answer**: Marko templates are more HTML-like compared to React’s JSX or Vue’s templates. It uses standard HTML with some additional templating syntax like `${}` for interpolation and simple event handlers. This makes it more approachable for developers familiar with HTML.

---

#### Q6: **How do you define a component in Marko?**

**Answer**: In Marko, components are defined in `.marko` files and are structured using the class syntax to add behaviors.

Example of a simple counter component:

```marko
class {
  onCreate() {
    this.state = { count: 0 };
  }

  increment() {
    this.state.count++;
  }
}

<div>
  <h1>Count: ${state.count}</h1>
  <button onClick('increment')>Increment</button>
</div>
```

---

#### Q7: **What is the use of the `input` object in Marko?**

**Answer**: The `input` object is used to pass data from the server or parent component to a Marko template. It allows dynamic values to be rendered in the template.

Example:

```marko
<h1>Hello ${input.name}!</h1>
```

---

#### Q8: **How does Marko handle conditional rendering?**

**Answer**: Marko allows you to use standard JavaScript for conditional rendering within the template using `if`, `else`, and `else-if`.

Example:

```marko
if (input.isLoggedIn) {
  <h1>Welcome back, ${input.name}!</h1>
} else {
  <h1>Welcome, Guest!</h1>
}
```

---

#### Q9: **How do you loop through an array in Marko?**

**Answer**: You can use the standard JavaScript `for` loop or `forEach` method to iterate over arrays and render them in the template.

Example:

```marko
<ul>
  <for(let item of input.items)>
    <li>${item}</li>
  </for>
</ul>
```

---

#### Q10: **How can you include a Marko component in another template?**

**Answer**: You can include a component in another Marko template using the `<${component}>` syntax.

Example:

```marko
<my-component message="Hello!" />
```

The component file `my-component.marko` will be included and rendered wherever it’s used.

---

### More Questions and Answers on Marko

#### Q11: **How does Marko handle client-side and server-side code together?**

**Answer**: Marko seamlessly handles both client-side and server-side rendering. Code that manipulates the DOM or responds to client-side events can be defined inside components, while the initial rendering happens on the server. Marko then progressively sends JavaScript to the client for interactivity.

---

#### Q12: **What is the difference between `res.render()` and `res.marko()` in an Express.js application using Marko?**

**Answer**: `res.marko()` is a method provided by the Marko-Express integration to render a Marko template. It is similar to `res.render()`, but specifically for Marko templates.

---

#### Q13: **How does Marko improve the performance of web applications?**

**Answer**: Marko improves performance by providing:

- **Server-side rendering**: Ensures that the first paint is fast.
- **Progressive rehydration**: Minimizes the amount of JavaScript sent to the client, allowing faster interaction.
- **Efficient templates**: Marko templates are lightweight and fast to parse.
