# Running Script

NPM scripts are commands defined in the package.json file that help automate tasks like testing, building, starting the server, and more. Scripts can be simple one-liners or more complex workflows involving multiple commands.

## Defining Scripts in `package.json`

Scripts are defined under the "scripts" field in `package.json`. You can run any script by using the npm run <script-name> command.

**Example `package.json` with Scripts:**

```json
{
  "name": "my-app",
  "version": "1.0.0",
  "scripts": {
    "start": "node app.js",
    "build": "webpack --config webpack.config.js",
    "test": "mocha",
    "lint": "eslint .",
    "dev": "nodemon app.js",
    "clean": "rm -rf dist"
  },
  "dependencies": {
    "express": "^4.17.1"
  },
  "devDependencies": {
    "nodemon": "^2.0.12",
    "webpack": "^5.0.0",
    "mocha": "^9.1.0",
    "eslint": "^7.32.0"
  }
}
```

## Running Scripts

You can run any of these scripts with the `npm run` command:

```
npm run <script-name>
```

**Example 1: Running the start Script**

To start the server by running the start script defined in `package.json`:

```
npm run start
```

Or, since `start` is a special script in NPM, you can simply run:

```
npm start
```

This will run the command `node app.js`.

**Example 2: Running the `build` Script**
To run the build process with Webpack, you would use:

```
npm run build
```

This executes the command `webpack --config webpack.config.js`.

**Example 3: Running the `test` Script**
To run tests using Mocha, defined in the `test` script:

```
npm run test
```

This will execute the `mocha` command to run your test suite.

## Predefined Script Hooks

NPM provides special "pre" and "post" hooks for scripts. These hooks are executed automatically before or after a script runs.

- **`pre<script>`**: Runs before the specified script.
- **`post<script>`**: Runs after the specified script.

**Example:**

```json
{
  "scripts": {
    "start": "node app.js",
    "prestart": "echo Pre-start hook",
    "poststart": "echo Post-start hook"
  }
}
```

Running `npm start` would produce:

```
Pre-start hook
Server started on port 3000
Post-start hook
```

## Running Scripts in Development Mode

When working in development mode, you may want to automatically restart your application when files change. You can use tools like `nodemon` for this.

**Example:**

```json
{
  "scripts": {
    "dev": "nodemon app.js"
  }
}
```

You can then run:

```
npm run dev
```

This starts the server and automatically restarts it whenever you make changes to the source code.

## Using Environment Variables in Scripts

You can pass environment variables to your scripts to control their behavior in different environments (development, production, etc.).

**Example:**

```json
{
  "scripts": {
    "start": "NODE_ENV=production node app.js",
    "dev": "NODE_ENV=development nodemon app.js"
  }
}
```

This sets the `NODE_ENV` variable, which you can access in your code like this:

```js
if (process.env.NODE_ENV === "production") {
  // Production-only settings
}
```

## Running a Script with `npx`

`npx` is a tool that comes with NPM, used to run Node.js packages without needing to install them globally. You can run a package directly from NPM without adding it to `node_modules`.

**Example:**

```
npx eslint .
```

This runs ESLint on your project files without needing to install it globally.

## Running Custom Commands

NPM scripts allow you to run any command-line utilities, not just Node.js-related commands.

**Example:**

```json
{
  "scripts": {
    "clean": "rm -rf dist",
    "deploy": "git push origin master"
  }
}
```

Here, `npm run clean` will delete the `dist` folder, and `npm run deploy` will push changes to the `master` branch on GitHub.

## Passing Arguments to NPM Scripts

You can pass arguments to scripts using `--` after the script name.

**Example:**

```json
{
  "scripts": {
    "build": "webpack"
  }
}
```

To pass arguments to `webpack`:

```
npm run build -- --mode production
```

This will run `webpack --mode production`.

## Executing Default Scripts

Certain scripts, like `start`, `test`, `restart`, and `stop`, are reserved and can be run without using the `run` keyword.

- `npm start`: Runs the `start` script.
- `npm test`: Runs the `test` script.
- `npm stop`: Runs the `stop` script.
- `npm restart`: Runs the `stop` script followed by the start script.

## Summary of Commands for Running Scripts:

- **Run a script**: `npm run <script-name>`
- **Run the `start` script**: `npm start`
- **Run the test script**: `npm test`
- **Run a script with arguments**: `npm run <script-name> -- <arguments>`
- **Run a package without installing**: `npx <package-name>`
