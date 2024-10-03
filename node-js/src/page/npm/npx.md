# npx

`npx` is a command-line tool that comes bundled with NPM (since version 5.2.0) and is used to execute Node.js packages directly without installing them globally or locally. It simplifies running executable packages from NPM by allowing you to run them without needing to manually install them beforehand.

Key Features of npx:

1. **Execute without installing**: Run a package from NPM without globally installing it.
1. **Runs local binaries**: It prioritizes using locally installed binaries over globally installed ones.
1. **Temporary installs**: It can temporarily install packages for one-time use, saving space.
1. **Version control**: Allows you to run a specific version of a package.
1. **Convenience**: Eliminates the need for installing packages globally just to use them once.

## Example Scenarios and Use Cases for npx

**1. Running a Package without Installing It Globally**

A common use case is running a package that you don't want to install globally. For example, running `create-react-app` to scaffold a new React project.

```
npx create-react-app my-app
```

Here, `npx` fetches the `create-react-app` package from the NPM registry, runs it to create a new React app in the `my-app` directory, and then cleans up afterward (without keeping `create-react-app` installed globally).

**2. Running Local Binaries**

Suppose you have a project with a local dependency like `eslint`:

```
npm install eslint --save-dev
```

You can run `eslint` using `npx` without needing to reference the full path of the binary inside `node_modules`:

```
npx eslint .
```

This command will find and execute the `eslint` binary in the locally installed `node_modules` folder, ensuring you're using the project's version of ESLint.

**3. Running a Specific Version of a Package**

You can use `npx` to run a specific version of a package. For example, if you want to run `http-server` version `0.12.0`:

```
npx http-server@0.12.0
```

This allows you to try different versions of a package without permanently installing them.

**4. Using Packages Temporarily**
If you need to quickly use a package like `nodemon` for testing, but don't want to install it globally, you can simply use:

```
npx nodemon app.js
```

`npx` will temporarily install `nodemon` (if it's not already installed), run it, and then remove it from the system after use.

**5. Avoiding Global Installation for Command-line Tools**

Some CLI tools (like `typescript`, `create-react-app`, `jest`, etc.) are often installed globally to use them across different projects. With `npx`, you can avoid global installs altogether and still run the CLI tool easily.

For example, to compile TypeScript without globally installing `tsc`:

```
npx tsc index.ts
```

**6. Using npx to Run GitHub Gists or Repositories**

You can also run commands from GitHub repositories directly using `npx`. For example, if someone has a CLI tool hosted in a GitHub repository, you can run it using the repository URL:

```
npx github:user/repo
```

This runs the tool directly from the GitHub repository without needing to clone the repository and manually install it.

## When to Use `npx`

- **Quick executions**: When you want to run a package for one-time use without installing it globally.
- **Version management**: When you need to try out or run a specific version of a package without updating your global installation.
- **Local binaries**: When running binaries installed locally within a project without using node_modules/.bin/.
