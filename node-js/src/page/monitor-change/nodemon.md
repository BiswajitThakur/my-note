# nodemon

**Nodemon** is a development tool that helps automatically restart a Node.js application when file changes in the directory are detected. It simplifies the development workflow by eliminating the need to manually stop and restart the server when changes are made to the codebase.

Nodemon watches the files in your project and restarts your application whenever a file changes. It's widely used in Node.js development to speed up the development process and ensure faster feedback on code changes.

### Key Features of Nodemon

1. **Automatic Restart**: Restarts your Node.js application when file changes are detected.
2. **File Monitoring**: Watches all the files in your project (by default) or specific files/directories.
3. **Customization**: You can specify which files or directories to watch or ignore.
4. **Works with Existing Code**: Nodemon is a drop-in replacement for `node`. It works with any Node.js-based application without requiring changes to your code.
5. **Configuration Options**: You can customize how Nodemon watches and restarts your application using configuration files or command-line options.
6. **Extensibility**: Nodemon can be extended with custom scripts to run before or after the application restarts.

### Installing Nodemon

To install Nodemon globally on your machine so you can use it from any project, run:

```bash
npm install -g nodemon
```

Or you can install it locally in your project and use it as a development dependency:

```bash
npm install --save-dev nodemon
```

### Using Nodemon

Once installed, you can replace the `node` command with `nodemon` to start your application:

```bash
nodemon app.js
```

This will start your `app.js` file and restart it automatically whenever changes are made to any files in the directory.

### Example: Basic Nodemon Usage

```bash
nodemon server.js
```

- This command will start `server.js`, and Nodemon will automatically restart the server when it detects changes to the files in your project.

### Example: Nodemon with a Custom Watch

You can specify specific directories or file extensions for Nodemon to watch by using the `--watch` flag:

```bash
nodemon --watch src --ext js,json server.js
```

This tells Nodemon to only watch the `src` directory and restart the application when `.js` or `.json` files change.

### Nodemon with Configuration File

You can also configure Nodemon using a `nodemon.json` configuration file:

```json
{
  "watch": ["src"],
  "ext": "js json",
  "ignore": ["public/*"],
  "exec": "node server.js"
}
```

In this configuration:

- Nodemon watches files in the `src` directory.
- It looks for `.js` and `.json` file extensions.
- It ignores changes in the `public` directory.
- The `exec` option specifies the command to run (`node server.js`).

### Command-Line Options for Nodemon

- **--watch**: Watch specific files or directories.

  ```bash
  nodemon --watch src server.js
  ```

- **--ext**: Specify which file extensions to watch.

  ```bash
  nodemon --ext js,json server.js
  ```

- **--ignore**: Ignore specific files or directories.

  ```bash
  nodemon --ignore logs/ server.js
  ```

- **--delay**: Delay the restart by a specified amount of time.

  ```bash
  nodemon --delay 2 server.js
  ```

- **--exec**: Use a custom command to start the application.
  ```bash
  nodemon --exec "npm run start" server.js
  ```

### Events and Custom Scripts

Nodemon allows you to hook into the start, restart, and exit events of the application using the `--exec` flag. For example, you can run a script before your application restarts:

```bash
nodemon --exec "echo 'File changed!'; node server.js"
```

You can also listen to events programmatically in your project. Here’s an example of handling restart events:

```javascript
nodemon({
  script: "app.js",
  ext: "js json",
}).on("restart", (files) => {
  console.log("App restarted due to changes in: ", files);
});
```

### Nodemon vs Node

- **Node**: To run your application with Node.js, you use the `node` command, but it won’t detect file changes, requiring manual restarts.
- **Nodemon**: When you use `nodemon`, it automatically restarts the application upon file changes, saving time during development.

### Example: Nodemon in a Project

In a typical Node.js project with Express.js:

```javascript
const express = require("express");
const app = express();

app.get("/", (req, res) => {
  res.send("Hello World!");
});

app.listen(3000, () => {
  console.log("Server is running on http://localhost:3000");
});
```

Running this project with:

```bash
nodemon server.js
```

Now, every time you make changes to the project files, Nodemon will automatically restart the server.

### Conclusion

Nodemon is an essential tool for Node.js development, enabling automatic restarts and speeding up the development process by detecting changes in the codebase. With its simplicity, customization options, and wide usage, Nodemon is a must-have for any Node.js developer.
