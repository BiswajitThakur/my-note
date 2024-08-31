# Modules

Node.js modules are reusable pieces of code that can be included and used in different parts of an application. They help in organizing code by breaking down functionality into smaller, manageable parts. Node.js comes with a set of core modules, and you can also create your own custom modules or use third-party modules from NPM (Node Package Manager).

## Types of Node.js Modules

### Core Modules:

Built-in and provided by Node.js. They do not need to be installed separately and can be used simply by requiring them in your application.

### Local Modules (Custom Modules):

Modules created by developers to organize code. These can be functions, classes, or objects that are bundled together and exported for use in other parts of an application.

Example:

```js
// math.js (Custom Module)
function add(a, b) {
  return a + b;
}
module.exports = add;

// app.js (Main File)
const add = require("./math");
console.log(add(2, 3)); // Outputs: 5
```

### Third-Party Modules:

Modules created by the community and published on NPM. These need to be installed via NPM and are used to extend the functionality of Node.js applications.
