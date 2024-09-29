# CommonJS

Node.js originally implemented the CommonJS module system, which is still widely used. In this system, each file in a Node.js application is considered a separate module. You use `require()` to load modules and `module.exports` to define what a module exports.

## Key Features:

- **`module.exports`**: Used to define the exported functionality of a module.
- **`require()`**: Used to import other modules.

## Example:

**math.js (Defining a Module):**

```js
// Defining a module with CommonJS
function add(a, b) {
  return a + b;
}

module.exports = add; // Export the function
```

**app.js (Using a Module):**

```js
// Importing the module using require()
const add = require("./math");

console.log(add(3, 4)); // Outputs: 7
```
