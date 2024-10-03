# [global] keyword

In Node.js, there is a global object called global, which is similar to window in browsers. It allows you to access global variables and functions from anywhere in your Node.js application. However, you should use global variables sparingly, as they can make the code harder to debug and maintain.

## Example

```js
// Declare a global variable
global.myGlobalVariable = "Hello, Global!";

// Function to access the global variable
function printGlobalVariable() {
  console.log(global.myGlobalVariable);
}

// Call the function
printGlobalVariable(); // Output: Hello, Global!

// You can also access standard Node.js global properties, e.g., process
console.log(global.process.platform); // Output: your OS platform (e.g., 'linux', 'darwin', 'win32')
```

In this example:

- myGlobalVariable is declared globally using global.myGlobalVariable.
- The function printGlobalVariable can access it from anywhere within the program.
- global also gives access to built-in objects like process, console, etc.

Using global variables is generally discouraged unless necessary, as it could lead to potential conflicts and unexpected behavior in larger applications. Instead, it's better to use local or module-scoped variables.

## Examples

### Example 1: Global Functions

You can also declare global functions using the global object.

```js
// Declare a global function
global.greet = function (name) {
  return `Hello, ${name}!`;
};

// Call the global function
console.log(global.greet("Alice")); // Output: Hello, Alice!
```

In this example:

- A global function greet is declared.
- It can be called from anywhere in the application using global.greet().

### Example 2: Accessing Built-in Globals

Node.js has some built-in global variables like process, Buffer, and setTimeout which are accessible through global.

```js
// Accessing global process object
console.log(global.process.version); // Node.js version

// Using setTimeout globally
global.setTimeout(() => {
  console.log("This will be printed after 2 seconds.");
}, 2000);
```

In this example:

- `global.process.version` gives the current Node.js version.
- `global.setTimeout()` works like `setTimeout()` and executes a callback after a delay.

### Example 3: Creating a Global Counter

```js
// Declare a global counter
global.counter = 0;

// Function to increment the counter globally
global.incrementCounter = function () {
  global.counter++;
  console.log(`Counter: ${global.counter}`);
};

// Call the function multiple times
incrementCounter(); // Output: Counter: 1
incrementCounter(); // Output: Counter: 2
incrementCounter(); // Output: Counter: 3
```

Here:

- `global.counter` stores a global counter.
- `The global.incrementCounter()` function increments and logs the value of counter.

### Example 4: Global Configuration

You can use the global object to store configuration data that's needed across multiple modules.

```js
// Declare global configuration
global.config = {
  appName: "MyApp",
  port: 3000,
};

// Access global configuration
console.log(`App Name: ${global.config.appName}`); // Output: App Name: MyApp
console.log(`Port: ${global.config.port}`); // Output: Port: 3000
```

In this example:

- A `config` object is stored globally.
- This configuration can be accessed from anywhere in the application.

### Example 5: Sharing Data Between Modules

Let's create two modules that share global data using global.

**Module 1: `module1.js`**

```js
// Initialize global data
global.sharedData = {
  username: "JohnDoe",
};

// Function to modify the global data
global.updateUsername = function (newUsername) {
  global.sharedData.username = newUsername;
};
```

**Module 2: `module2.js`**

```js
// Access the global data
console.log(`Initial Username: ${global.sharedData.username}`); // Output: Initial Username: JohnDoe

// Update the global data
global.updateUsername("JaneDoe");

// Access the updated global data
console.log(`Updated Username: ${global.sharedData.username}`); // Output: Updated Username: JaneDoe
```

In this example:

- `global.sharedData` is initialized in `module1.js` and is updated using the `global.updateUsername()` function.
- `module2.js` accesses the same global data and can see the updates made in module1.js.

## Basic Interview Questions

#### 1. What is the global object in Node.js, and how is it different from the window object in the browser?

The `global` object in Node.js is similar to the `window` object in browsers, but while `window` is specific to the browser environment, `global` is specific to Node.js. `global` provides access to global variables, objects, and functions, such as `process`, `console`, and `setTimeout`. In the browser, `window` refers to the current tab or window, while in Node.js, `global` refers to the entire global scope of the application.

#### 2. How can you declare a `global` variable in Node.js?

You can declare a global variable using the global object. For example:

```js
global.myGlobalVar = "Hello, world!";
```

#### 3. What are some of the built-in objects that are available in the `global` object in Node.js?

Some built-in objects available in the global object include:

- `process`
- `Buffer`
- `setTimeout`, `setInterval`
- `clearTimeout`, `clearInterval`
- `console` These objects can be accessed directly or through `global`.

#### 4. Is it necessary to prefix global functions or variables with `global` when using them in your Node.js application?

No, it is not necessary to prefix global functions or variables with `global` because they are already available globally. However, doing so makes it explicit that the variable or function is part of the global scope.

## Intermediate Questions

#### 5. What are the potential risks of using the `global` object in Node.js?

Using the `global` object can lead to:

- **Namespace pollution**: Conflicts can arise if multiple parts of the application or modules define the same global variable name.
- **Tight coupling**: Over-reliance on global variables can lead to tightly coupled code, making it harder to maintain, test, and debug.
- **Unexpected behavior**: If a global variable is modified in one part of the application, it may unintentionally affect other parts of the code that also rely on it.

#### 6. How can you avoid global namespace pollution in Node.js?

You can avoid global namespace pollution by:

- Using module-scoped variables instead of global variables.
- Using closures to keep variables local to a specific function or module.
- Avoiding the use of global wherever possible and instead passing variables explicitly between modules or functions.

#### 7. If you define a global variable in one module, can it be accessed in another module without exporting it? Explain.

Yes, if you define a global variable using global in one module, it can be accessed in any other module without needing to export it. The global object is shared across all modules, so variables and functions attached to global are accessible globally.

```js
// In module1.js
global.sharedData = "This is global data";

// In module2.js
console.log(global.sharedData); // Output: This is global data
```

#### 8. Explain the difference between local, module-scoped, and global variables in Node.js.

- **Local variables**: Declared inside a function and are only accessible within that function.
- **Module-scoped variables**: Declared at the module level (outside functions) and are only accessible within that specific module unless explicitly exported.
- **Global variables**: Declared using global, and they can be accessed from any module in the Node.js application.

## Advanced Questions

#### 9. How does the global object affect the performance and scalability of a Node.js application?

Overusing the global object can negatively affect performance and scalability:

- **Memory leakage**: If global variables are not properly cleaned up, they can lead to memory leaks, as they persist throughout the application's lifetime.
- **Concurrency issues**: In a highly concurrent application, global state can introduce bugs if different parts of the code modify global variables simultaneously.
- **Testability**: Global variables can make unit testing difficult, as the state is shared across the entire application, making tests dependent on each other.

#### 10. How can you mitigate the risks associated with global state when building large applications in Node.js?

To mitigate risks:

- Minimize the use of the global object. Use module exports and function parameters to pass data between modules.
- Use dependency injection to manage shared resources instead of relying on global state.
- Encapsulate state within classes or functions to avoid pollution of the global namespace.
- Use tools like proxyquire or mock-require for safely mocking global variables during testing.

#### 11. Explain how the global object interacts with JavaScript event loop mechanisms like setTimeout or setImmediate.

Answer: The global object provides access to functions like setTimeout, setInterval, and setImmediate, which are used to schedule code execution in the event loop. These functions allow asynchronous operations to be executed after a delay (setTimeout, setInterval) or at the next available opportunity after the current event loop phase (setImmediate). These timing functions are globally available because they are part of Node.js's core functionality for managing asynchronous tasks.

#### 12. Can you modify built-in global objects like console or process in Node.js? Is it a good practice?

Technically, you can modify built-in global objects like console or process. For example, you could override console.log:

```js
global.console.log = function (message) {
  process.stdout.write(`[LOG]: ${message}\n`);
};
```

However, modifying built-in globals is generally not recommended because it can lead to unexpected behavior across the entire application, especially in libraries or modules that rely on the default behavior.

#### 13. How does Node.js handle memory management for global variables, and how can improper usage of global lead to memory leaks?

Node.js relies on V8's garbage collector for memory management. The garbage collector frees memory by removing objects that are no longer referenced in the application. Global variables, however, are always referenced because they are part of the global object and stay in memory throughout the entire application's lifecycle.

If you create large objects or data structures globally and don't remove them when they're no longer needed, they will persist in memory, leading to memory leaks. For example, large datasets stored in global variables could keep growing, causing the application to use more memory over time, eventually affecting performance.

To avoid memory leaks:

- Remove unnecessary global references explicitly (e.g., delete global.someVariable).
- Use module-scoped or local variables to limit their lifespan.
- Use tools like memory profiling to detect leaks during development.

#### 14. How can you ensure that your global variables don't conflict with other global variables in a complex Node.js application with many modules or third-party dependencies?

In large applications or projects that rely on multiple modules and libraries, global variable conflicts can arise if multiple parts of the codebase or third-party dependencies define the same global variable name.

To avoid this:

- **Namespace your global variables**: Instead of defining a global variable directly, use a namespace. For example:

```js
global.myApp = global.myApp || {};
global.myApp.config = {
  appName: "MyApplication",
};
```

- **Avoid using global for state sharing**: Instead, encapsulate shared data in modules and export them. This way, each module has its own isolated scope, reducing the likelihood of conflicts:

```js
// module1.js
const config = { appName: "MyApp" };
module.exports = config;

// module2.js
const config = require("./module1");
console.log(config.appName); // MyApp
```

- **Use Dependency Injection**: Instead of relying on shared global state, pass dependencies as arguments to your functions or modules.

#### 15. What are the security risks of using global variables in a Node.js application, especially when dealing with user inputs?

Using global variables carelessly can introduce several security risks, especially when dealing with untrusted or user-supplied data. The primary risks include:

- **Global State Manipulation**: If user inputs can modify or overwrite global variables, this could lead to unintended behavior across the application. For example, a user might exploit a bug to overwrite a global configuration variable and disable security checks.

Example:

```js
global.isAuthenticated = false; // Default value

function login(username, password) {
  if (username === "admin" && password === "password") {
    global.isAuthenticated = true;
  }
}

// If this value can be tampered with, anyone can bypass authentication checks.
```

- **Prototype Pollution**: Malicious users can manipulate JavaScript object prototypes, which could result in security vulnerabilities. If you expose global objects or functions that allow object modification, a user could potentially inject harmful code.

Example:

```js
Object.prototype.foo = "bar"; // A malicious addition
console.log({}.foo); // Output: bar
```

- **Injection Attacks**: If global variables store user inputs and these inputs are later used in unsafe ways (e.g., dynamic code execution), it could lead to injection attacks like command injection or SQL injection.

**Mitigations:**

- Never store sensitive data like user credentials, tokens, or secrets in global variables.
- Always sanitize and validate user inputs.
- Limit access to global state and use module or function scope for critical variables.
- Use security best practices like input validation libraries (e.g., validator.js), avoiding dynamic code execution (eval()), and regularly updating Node.js and its dependencies.

#### 16. How does Node.js handle the global execution context across multiple requests in a server application, and what are the pitfalls of using global in a multi-request environment?

In Node.js, each request to a server runs in a separate event loop iteration, but all requests share the same global execution context. This means that if you use the global object to store request-specific data, it could result in unexpected behavior, especially in concurrent applications.

For example:

```js
global.currentUser = "User1"; // Set during request 1
// Request 2 could overwrite the global state:
global.currentUser = "User2"; // Set during request 2

// If request 1 is still processing, it might mistakenly use the data from request 2.
```

This can cause race conditions and data inconsistency across requests.

**Pitfalls**:

- **Race Conditions**: When multiple requests update the same global variable, there is no guarantee about the order in which these updates occur.
- **Data Leakage**: One user's data could accidentally be exposed to another user if global state is improperly shared.

**Mitigations**:

- Avoid storing request-specific data in global. Use request-scoped objects instead, like local variables or middleware-specific storage in frameworks like Express.
- Use technologies like async_hooks in Node.js to track the lifecycle of asynchronous operations and handle request-specific data in a controlled manner.

#### 17. What are some performance considerations when frequently accessing or modifying global variables in Node.js?

Accessing global variables frequently can have performance implications due to the following reasons:

- **Lookup Cost**: Accessing variables in the global scope takes longer compared to local variables because JavaScript has to traverse the scope chain to resolve the variable. This can introduce a performance overhead, especially in performance-critical sections of the code (e.g., inside loops).

- **Memory Consumption**: Global variables persist for the entire lifetime of the application. If the application frequently creates large objects or datasets globally and doesn't clean them up, it can lead to high memory consumption, increasing garbage collection pressure and eventually leading to degraded performance.

- **Garbage Collection**: Since global variables are never garbage-collected unless explicitly removed, they stay in memory, causing more frequent and longer garbage collection pauses as memory usage increases.

**Mitigations**:

- **Minimize Global Access**: Limit the use of global variables. Keep frequently accessed variables in local or module scope to reduce lookup overhead.
- **Clean Up Unused Globals**: When a global variable is no longer needed, explicitly remove it using delete global.variableName.
- **Use Local Scope in Hot Code Paths**: If a global variable is accessed multiple times in a performance-critical section, store it in a local variable:

```js
const myVar = global.someVar; // Cache global value locally
for (let i = 0; i < 1000; i++) {
  // Use myVar instead of global.someVar
}
```

#### 18. How can async_hooks help you track global variable usage across asynchronous operations in Node.js?

async_hooks is a core module in Node.js that allows developers to track the lifecycle of asynchronous operations such as promises, callbacks, and timers. This is especially useful for managing global state or tracking request-specific data across asynchronous boundaries.

For example, in a web server, async_hooks can be used to track which asynchronous operations belong to a specific request, allowing you to manage state more effectively without relying on global variables.

Here's a simple use case with async_hooks:

```js
const async_hooks = require("async_hooks");

const store = new Map();

async_hooks
  .createHook({
    init(asyncId, type, triggerAsyncId) {
      const parentStore = store.get(triggerAsyncId);
      if (parentStore) {
        store.set(asyncId, parentStore);
      }
    },
    destroy(asyncId) {
      store.delete(asyncId);
    },
  })
  .enable();

function setRequestContext(requestId) {
  store.set(async_hooks.executionAsyncId(), { requestId });
}

function getRequestContext() {
  return store.get(async_hooks.executionAsyncId());
}

// Example usage
setRequestContext("123");
console.log(getRequestContext()); // Output: { requestId: "123" }
```

Using async_hooks, you can track asynchronous tasks and ensure request-specific data is handled without relying on globals, which prevents issues like data leakage or state corruption across requests.
