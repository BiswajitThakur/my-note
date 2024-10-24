# [`pug`](https://pugjs.org/api/getting-started.html)

**Pug** is a high-performance template engine heavily used in Node.js applications. It’s an HTML preprocessor that simplifies the creation of HTML documents by allowing you to write more concise and readable code. Pug uses indentation instead of closing tags, reducing verbosity.

---

### Features of Pug:

1. **Clean and concise syntax**: No need for closing tags or quotes around attributes.
2. **Built-in support for loops, conditionals, and includes**: You can easily manage dynamic content.
3. **HTML abstraction**: It helps avoid repetition and reduces complexity in your views.
4. **Whitespace and indentation sensitive**: Pug uses whitespace to determine structure instead of curly braces or parentheses.

---

### Installation of Pug

To use Pug in your Node.js application, install it using npm:

```bash
npm install pug
```

---

### Setting up Pug with Express.js

Here’s how you can set up Pug as the view engine in an Express.js application.

#### Step 1: Install Dependencies

```bash
npm install express pug
```

#### Step 2: Set Up Express.js to Use Pug

```javascript
// app.js

const express = require("express");
const app = express();

// Set Pug as the view engine
app.set("view engine", "pug");

// Route that renders a Pug template
app.get("/", (req, res) => {
  res.render("index", { title: "Welcome", message: "Hello, World!" });
});

// Start the server
app.listen(3000, () => {
  console.log("Server is running on http://localhost:3000");
});
```

#### Step 3: Create a Pug Template

In the `views` folder, create a file called `index.pug`:

```pug
doctype html
html
  head
    title= title
  body
    h1= message
```

#### Step 4: Run the Server

Run your Express.js server:

```bash
node app.js
```

Navigate to `http://localhost:3000` and you will see "Hello, World!" displayed on the page.

---

### Syntax of Pug

#### Variables

Pug allows you to interpolate variables in HTML. For instance:

```pug
h1 Welcome, #{name}
```

This will render:

```html
<h1>Welcome, John</h1>
```

#### Loops

Loops are supported using JavaScript’s `for` syntax:

```pug
ul
  each item in items
    li= item
```

This will render a list of items passed to the template.

#### Conditionals

You can use if/else conditionals to control what gets rendered:

```pug
if user
  h1 Welcome back, #{user.name}!
else
  h1 Welcome, Guest!
```

---

### Example with Pug Conditionals and Loops

```pug
doctype html
html
  head
    title User List
  body
    h1 User List
    ul
      each user in users
        li
          if user.active
            span= user.name
          else
            span Inactive User
```

If you pass a list of users as data, this will render a list of users with a label indicating whether they are active or inactive.

---

### Questions and Answers on Pug

#### Q1: **What is Pug and how does it work in Node.js?**

**Answer**: Pug is a high-performance template engine for Node.js, used to generate HTML dynamically. It provides a simpler and more readable syntax compared to HTML by using indentation to indicate nesting. Pug is commonly used with frameworks like Express.js for rendering views.

---

#### Q2: **How do you set up Pug as the view engine in Express.js?**

**Answer**:

1. Install Pug with `npm install pug`.
2. Set it as the view engine in Express.js using `app.set('view engine', 'pug')`.
3. Use `res.render('templateName', data)` to render Pug templates with data.

Example:

```javascript
app.set("view engine", "pug");
app.get("/", (req, res) => {
  res.render("index", { title: "Hello", message: "Welcome!" });
});
```

---

#### Q3: **What are the key differences between Pug and HTML?**

**Answer**:

- Pug eliminates the need for closing tags.
- Attributes in Pug don’t require quotes.
- It uses indentation for nested elements instead of curly braces or tags.
- Pug allows embedding JavaScript logic directly in the templates, such as loops and conditionals.

---

#### Q4: **How can you include one Pug file inside another?**

**Answer**: Pug allows you to include reusable templates by using the `include` directive. This helps in maintaining DRY (Don't Repeat Yourself) principles.

Example:

```pug
include header.pug
h1 Welcome to My Site
include footer.pug
```

---

#### Q5: **How do you pass variables to a Pug template in Express.js?**

**Answer**: Variables are passed as an object to the `res.render()` method. Inside the Pug template, you can reference them using `=`.

Example:

```javascript
app.get("/", (req, res) => {
  res.render("index", { name: "John", age: 30 });
});
```

In the Pug template:

```pug
h1= name
p Age: #{age}
```

---

#### Q6: **What is the purpose of `extends` in Pug?**

**Answer**: The `extends` keyword allows you to inherit a base layout or template in Pug. This is useful for creating consistent layouts (such as headers, footers) across multiple pages.

Example:

```pug
//- layout.pug
doctype html
html
  head
    title My Website
  body
    block content
```

```pug
//- index.pug
extends layout.pug

block content
  h1 Welcome to my page
```

---

#### Q7: **What is the difference between `=` and `-` in Pug?**

**Answer**:

- `=` is used to output (escape) the value of a variable or expression into the HTML.
- `-` is used for executing JavaScript logic without outputting the result.

---

#### Q8: **How can you conditionally render elements in Pug?**

**Answer**: Pug allows conditional rendering using JavaScript's `if` and `else` statements directly within the template.

Example:

```pug
if user
  h1 Welcome, #{user.name}!
else
  h1 Welcome, Guest!
```

---

#### Q9: **How does Pug handle loops for rendering lists?**

**Answer**: You can use JavaScript’s `for` or `each` syntax to iterate through arrays and render each element.

Example:

```pug
ul
  each item in items
    li= item
```

---

#### Q10: **How do you escape HTML in Pug templates to prevent XSS attacks?**

**Answer**: By default, Pug escapes the content of variables when using `=`. If you want to output raw HTML without escaping, you would use `!=`.

Example:

```pug
p= '<script>alert("XSS")</script>' // This will be escaped
p!= '<script>alert("XSS")</script>' // This will be rendered as raw HTML
```

---

### More Questions and Answers on Pug

#### Q11: **How do you handle loops with an index in Pug?**

**Answer**: You can access the index of the loop by modifying the `each` syntax:

```pug
ul
  each item, index in items
    li= index + ': ' + item
```

---

#### Q12: **Can you write inline JavaScript in Pug?**

**Answer**: Yes, Pug allows you to embed JavaScript directly in the template using the `-` syntax.

Example:

```pug
- var today = new Date();
p Today is #{today.toDateString()}
```

---

#### Q13: **How do you create comments in Pug?**

**Answer**: Comments in Pug can be written using `//` for normal comments and `//-` for unrendered comments (comments that won’t appear in the output HTML).

Example:

```pug
// This is a normal comment and will be in the HTML
//- This comment will not appear in the output HTML
```

---

### Conclusion

Pug simplifies HTML creation by offering a cleaner syntax, making it easy to maintain, reuse, and extend templates. Its integration with JavaScript for dynamic content generation makes it an excellent choice for web applications, especially when used with Node.js and Express.
