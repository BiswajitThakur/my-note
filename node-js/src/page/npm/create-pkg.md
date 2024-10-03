# Creating Packages

NPM packages are reusable modules of code that you can publish and distribute to others via the NPM registry, or just use within your own projects. Here’s how you can create and publish an NPM package step by step.

## Initialize a New NPM Package

To create an NPM package, first, you need to initialize a project. This creates a `package.json` file that holds metadata about the package (name, version, description, dependencies, etc.).

**Command:**

```bash
npm init
```

This will prompt you to answer several questions like the package name, version, description, entry point, and so on. After answering the prompts, `package.json` is created.

Alternatively, if you want to skip the prompts and use default values, you can run:

```bash
npm init -y
```

## Set Up the Package Structure

The structure of an NPM package typically looks like this:

```
my-package/
├── package.json
├── index.js
└── README.md
```

- `index.js`: This is the main entry point of the package, which you define in package.json.
- `package.json`: Contains metadata and dependency information.
- `README.md`: A markdown file that provides documentation about the package.

**Example `index.js`:**

```js
function sayHello(name) {
  return `Hello, ${name}!`;
}

module.exports = sayHello;
```

In this example, the function `sayHello` is the main functionality that will be exported by the package.

## Defining package.json

The `package.json` file includes important information about your package, such as its name, version, description, entry point, and more.

**Example `package.json`:**

```json
{
  "name": "my-awesome-package",
  "version": "1.0.0",
  "description": "A simple package that greets people",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": ["greeting", "hello", "example"],
  "author": "Your Name",
  "license": "MIT"
}
```

**Key fields:**

- **`name`**: The name of your package. Must be unique in the NPM registry if you plan to publish it.
- **`version`**: Follows Semantic Versioning (major.minor.patch). It’s important to update this as your package evolves.
- **`main`**: The entry point of your package (e.g., index.js).
- **`keywords`**: Useful for searchability on the NPM registry.
- **`license`**: Specifies the license under which your package is distributed.

## Writing a README.md

A good `README.md` file is essential to provide clear documentation to users. It should include:

- **Introduction**: A brief description of what the package does.
- **Installation**: Instructions on how to install the package.
- **Usage**: Examples of how to use the package.
- **Contributing**: (Optional) Guidelines for contributing to the package.
- **License**: Information about the package’s license.

**Example `README.md`:**

````md
# My Awesome Package

This is a simple package that greets people.

## Installation

```
npm install my-awesome-package
```

## Usage

```js
const sayHello = require("my-awesome-package");

console.log(sayHello("World")); // Outputs: Hello, World!
```

## License

MIT License
````

## Testing Your Package Locally

Before publishing, you can test your package locally by linking it to a test project.

#### A. Link the Package Globally:

In the directory of your package, run:

```bash
npm link
```

#### B. Use the Package in Another Project:

In a separate project, you can link to your package by running:

```bash
npm link my-awesome-package
```

This lets you use the package as if it were installed from NPM, allowing you to test it in real-world scenarios before publishing.

## Publishing Your Package to NPM

Once your package is ready, you can publish it to the NPM registry.

#### A. Login to NPM:

First, log in to your NPM account (if you don’t have an account, [create one here](https://www.npmjs.com/signup)):

```bash
npm login
```

You’ll be prompted for your username, password, and email.

#### B. Publish the Package:

After logging in, simply run:

```bash
npm publish
```

If the package name is unique and there are no errors, your package will be published to the NPM registry.

#### C. Access Your Package:

Once published, your package can be installed by others via:

```bash
npm install my-awesome-package
```

## Versioning Your Package

To publish updates to your package, you need to update its version following Semantic Versioning.

**Patch version (x.x.X)**: Bug fixes, no backward-incompatible changes.
**Minor version (x.X.x)**: New features, no breaking changes.
**Major version (X.x.x)**: Breaking changes or major new functionality.

Use the `npm version` command to bump the version:

```bash
npm version patch   # 1.0.1 -> 1.0.2
npm version minor   # 1.0.2 -> 1.1.0
npm version major   # 1.1.0 -> 2.0.0
```

This updates the version in `package.json` and creates a new Git tag (if Git is initialized).

Then, you can publish the updated package:

```bash
npm publish
```

## Unpublishing a Package

If you need to remove a package from the NPM registry, you can unpublish it. However, use this with caution, especially for widely used packages.

To unpublish a package:

```bash
npm unpublish <package-name> --force
```

## Scoped Packages

Scoped packages are namespaced under a specific user or organization. This is useful when the package name you want is already taken or if you’re working under an organization.

- Scoped package example: `@your-username/my-package`

Create a Scoped Package:

1. Add a scope to the `name` in `package.json`:

```json
{
  "name": "@your-username/my-awesome-package"
}
```

2. Publish a scoped package:

```bash
npm publish --access public
```

## Best Practices for NPM Packages

- **Clear Documentation**: Provide a clear and concise `README.md` to help users understand how to use your package.
- **Proper Versioning**: Use Semantic Versioning (`semver`) for managing updates.
- **Security**: Regularly check for vulnerabilities using `npm audit` and follow best practices for dependencies.
- **Testing**: Always test your package before publishing. Consider using CI/CD for automated testing.
- **Licensing**: Include a license to clarify usage rights.

## Summary

- Initialize the package: `npm init`
- Write your code in an entry file like `index.js`.
- Test locally using `npm link`.
- Publish to NPM using `npm publish`.
- Version updates with `npm version`.
