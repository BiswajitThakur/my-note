# npm

**NPM (Node Package Manager)** is a package manager for JavaScript, specifically designed to work with Node.js. It allows developers to manage dependencies (external libraries or modules) for their projects and provides a registry where thousands of reusable packages are hosted. NPM helps in installing, updating, and managing these packages.

NPM has two main components:

1. **Command-line tool (CLI)**: A tool for interacting with the NPM ecosystem, performing actions such as installing and managing packages.
1. **Online registry**: A public database of JavaScript packages, where developers can publish their libraries or download existing ones.

**Basic NPM Commands**:

1. **`npm init`**: Initializes a new Node.js project by creating a `package.json` file, which tracks the project’s metadata and dependencies.
1. **`npm install`**: Installs packages and adds them to the `node_modules` folder. It can also save dependencies in `package.json`.
1. **`npm update`**: Updates installed packages to the latest versions.
1. **`npm publish`**: Publishes your own package to the NPM registry.

## Example: Setting Up a Project with NPM

**Step 1: Initialize a Node.js Project**

First, navigate to your project folder and initialize a new Node.js project using `npm init`.

```bash
mkdir my-npm-project
cd my-npm-project
npm init
```

This command will prompt you to answer several questions about the project. It will create a `package.json` file that looks like this:

```json
{
  "name": "my-npm-project",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [],
  "author": "",
  "license": "ISC"
}
```

**Step 2: Installing a Package**

You can now install packages (modules) from the NPM registry. For example, let's install the popular package Lodash (a utility library).

```
npm install lodash
```

This will:

- Download the package from the NPM registry.
- Add the package to the `node_modules` folder.
- Update package.json to include the dependency.

The `package.json` file will now include:

```json
{
  "name": "my-npm-project",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": [],
  "author": "",
  "license": "ISC",
  "dependencies": {
    "lodash": "^4.17.21"
  }
}
```

`lodash` is now listed under `dependencies`, meaning it’s a required package for the project.

**Step 3: Using the Installed Package**

Now you can use the installed package in your JavaScript code. Create a file named index.js in the root directory of the project:

```js
// index.js
const _ = require("lodash");

// Using Lodash to chunk an array
const array = [1, 2, 3, 4, 5, 6];
const chunkedArray = _.chunk(array, 2);
console.log(chunkedArray); // Output: [ [ 1, 2 ], [ 3, 4 ], [ 5, 6 ] ]
```

Run the code:

```
node index.js
```

This should print the chunked array to the console.

**Step 4: Installing Development Dependencies**

Some packages are only needed during development, such as testing libraries. You can install them using the `--save-dev` flag:

```
npm install --save-dev jest
```

This adds the package to the `devDependencies` section of `package.json`:

```json
{
  "devDependencies": {
    "jest": "^29.4.0"
  }
}
```

Development dependencies are only installed when running npm install in a development environment, not in production.

**Step 5: Running Scripts**
You can define custom scripts in `package.json`. For example, to use Jest for running tests, modify `scripts` like this:

```json
"scripts": {
  "test": "jest"
}
```

Now, you can run your test suite by typing:

```
npm test
```
