# Custom Modules

Custom Modules in Node.js are user-defined modules that you create yourself, as opposed to built-in modules (like fs, http) or third-party modules (installed via npm). These modules allow you to organize your code into smaller, reusable, and maintainable blocks.

In Node.js, each file is treated as a separate module, and you can define exports from one file to be used in another using either the CommonJS system (with module.exports and require()) or the ESM system (with export and import).

## 1. Creating a Custom Module (CommonJS)

To create a custom module in Node.js, you simply write code in one file and export the parts you want to make available to other files using module.exports. In the consuming file, you use require() to import the module.

**Example:**
`math.js` (Custom Module):

```js
// Define a custom function
function add(a, b) {
  return a + b;
}

// Export the function using module.exports
module.exports = add;
```

`app.js` (Main File Using the Custom Module):

```js
// Import the custom module using require()
const add = require("./math"); // './math' refers to math.js in the same directory

console.log(add(3, 5)); // Output: 8
```

**Here’s how the flow works:**

- The math.js file defines a function add() and exports it using module.exports.
- The app.js file imports the function using require() and then calls it.

### Exporting Multiple Functions/Variables:

You can export multiple functions, objects, or variables by attaching them to module.exports.

**`math.js` (Exporting Multiple Functions):**

```js
// Define multiple functions
function add(a, b) {
  return a + b;
}

function subtract(a, b) {
  return a - b;
}

// Export both functions as an object
module.exports = {
  add,
  subtract,
};
```

**`app.js` (Using Multiple Exports):**

```js
// Import the custom module
const math = require("./math");

console.log(math.add(3, 5)); // Output: 8
console.log(math.subtract(9, 4)); // Output: 5
```

## 2. Creating a Custom Module (ESM)

With ESM (ECMAScript Modules), you can use export and import instead of module.exports and require(). ESM is the modern way to handle modules in JavaScript and is becoming more common in Node.js.

**Example:**
**`math.mjs` (Custom Module):**

```js
// Define functions
export function add(a, b) {
  return a + b;
}

export function subtract(a, b) {
  return a - b;
}
```

**`app.mjs` (Main File Using the Custom Module):**

```js
// Import functions using ESM syntax
import { add, subtract } from "./math.mjs";

console.log(add(3, 5)); // Output: 8
console.log(subtract(9, 4)); // Output: 5
```

**Default Exports in ESM:**

You can also define a default export in ESM, which allows the module to export a single value, function, or object by default.

**math.mjs (Using Default Export):**

```js
// Exporting a default function
export default function add(a, b) {
  return a + b;
}
```

**`app.mjs`:**

```js
// Importing the default export
import add from "./math.mjs";

console.log(add(3, 5)); // Output: 8
```

## 3. Directory as a Custom Module

In Node.js, you can structure modules into directories. If a directory contains a package.json file with a "main" field, Node.js will treat the entire directory as a module.

**Example:**
**Folder Structure:**

```
/math
  ├── add.js
  ├── subtract.js
  └── index.js
```

**`add.js` :**

```js
// Define add function
function add(a, b) {
  return a + b;
}

module.exports = add;
```

**`subtract.js` :**

```js
// Define subtract function
function subtract(a, b) {
  return a - b;
}

module.exports = subtract;
```

**`index.js`:**

```js
// Import functions from add.js and subtract.js
const add = require("./add");
const subtract = require("./subtract");

// Export them together as a single module
module.exports = {
  add,
  subtract,
};
```

**`app.js`:**

```js
// Import the entire directory as a module
const math = require("./math"); // This refers to the index.js inside the math directory

console.log(math.add(3, 5)); // Output: 8
console.log(math.subtract(9, 4)); // Output: 5
```
