# Installing Packages

**NPM (Node Package Manager)** is used to install, update, and manage third-party libraries and dependencies for your project. Here's how you can install packages using NPM:

## Installing a Package Locally

A locally installed package is available only within your project. It is placed inside the `node_modules` folder and listed in the `package.json` file.

To install a package locally, use the following command:

```
npm install <package-name>
```

#### Example

```
npm install express
```

This installs the `express` package locally and adds it as a dependency in your `package.json` file if it exists.

After installation, you can use `express` in your project like this:

```js
const express = require("express");
const app = express();
```

**Flags You Can Use:**

- `--save`: Automatically adds the package to your package.json file (this is the default behavior starting from NPM v5).
- `--save-dev`: Installs the package as a development dependency, useful for packages needed only during development, like testing tools or build tools.

**Example**

```
npm install mocha --save-dev
```

## Installing a Package Globally

A globally installed package can be accessed from any directory on your system. This is useful for command-line tools that you need across multiple projects.

To install a package globally, use the `-g` flag:

```
npm install -g <package-name>
```

**Example**

```
npm install -g nodemon
```

Now, `nodemon` can be used globally from the command line:

```
nodemon app.js
```

## Installing Specific Versions

You can install a specific version of a package by specifying the version number:

```
npm install <package-name>@<version>
```

**Example:**

```
npm install lodash@4.17.15
```

This installs version `4.17.15` of the lodash library.

## Installing Multiple Packages at Once

You can install multiple packages in one command by listing them all:

```
npm install express mongoose cors
```

This will install the express, mongoose, and cors packages in a single command.

## Installing from `package.json`

If a `package.json` file exists in your project directory, you can install all listed dependencies with:

```
npm install
```

This installs all the dependencies defined under `"dependencies"` and `"devDependencies"` in the `package.json` file.

## Installing Packages from GitHub

You can also install packages directly from GitHub repositories:

```
npm install git+https://github.com/user/repo.git
```

Or from a specific commit or branch:

```
npm install git+https://github.com/user/repo.git#branch-name
```

**Example**

```
npm install git+https://github.com/expressjs/express.git
```

## Installing Packages as Peer Dependencies

Peer dependencies are packages that the package you're installing relies on but expects the consuming project to install directly.

To install a package and list it as a peer dependency, use the `--peer` flag:

```
npm install <package-name> --peer
```

## Common NPM Commands for Managing Packages

- **View Installed Packages**: List all locally installed packages.

```
npm list
```

- **Uninstalling a Package**: Remove a locally installed package.

```
npm uninstall <package-name>
```

- **Updating a Package**: Update a package to the latest version.

```
npm update <package-name>
```

- **Reinstalling Dependencies**: If you want to reinstall everything (useful for resetting the environment):

```
rm -rf node_modules
npm install
```

## Summary

- **Local Installation**: `npm install <package-name>` installs a package for use within the project.
- **Global Installation**: npm install -g <package-name> installs a package globally so you can use it from anywhere.
- **Development Dependencies**: Use `--save-dev` for packages needed during development.
- **Specific Version**: Install specific versions using `npm install <package-name>@<version>`.
