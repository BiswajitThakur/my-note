# ESM (ECMAScript Modules)

With ES6 (ECMAScript 2015), JavaScript introduced ESM (ECMAScript Modules), which is now a standard for both browser and server environments like Node.js. ESM uses import to bring in modules and export to expose them.

## Key Features:

- **export**: Declares what functionality a module exports (either a named or default export).
- **import**: Imports modules from another file.

## Example:

**math.mjs (Defining a Module with ESM):**

```js
// Defining a module using ESM
export function add(a, b) {
  return a + b;
}
```

**app.mjs (Using a Module with ESM):**

```js
// Importing the module using ESM
import { add } from "./math.mjs";

console.log(add(3, 4)); // Outputs: 7
```
