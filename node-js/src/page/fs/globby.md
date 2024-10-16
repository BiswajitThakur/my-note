# [`globby`](https://www.npmjs.com/package/globby)

**Globby** is a user-friendly wrapper around the `fast-glob` library that simplifies the process of matching file paths using glob patterns. It enhances the globbing experience by adding features such as better defaults, support for multiple patterns, and improved handling of symlinks.

### Key Features of Globby

1. **Promise-based API**: Globby supports both callback and promise-based approaches, making it easier to integrate with modern JavaScript async/await syntax.

2. **Multiple Patterns**: You can pass multiple glob patterns to a single globby call, allowing for more flexible and efficient file matching.

3. **Improved Defaults**: Globby provides sensible defaults, reducing the need for configuration.

4. **Support for `dot` and `gitignore`**: It allows for matching hidden files and automatically respects `.gitignore` files, which is particularly useful for projects using Git.

5. **`expandDirectories` option**: This feature helps expand directories into their contents when matched.

### Installation

You can install Globby via npm:

```bash
npm install globby
```

### Basic Usage Example

Here’s a simple example of how to use Globby to match files with various patterns:

```javascript
const globby = require("globby");

async function run() {
  // Define the glob patterns
  const patterns = [
    "*.js", // Match all JavaScript files in the current directory
    "src/**/*.js", // Match all JavaScript files in the src directory and its subdirectories
    "!**/node_modules/**", // Exclude node_modules directory
  ];

  // Use globby to get matching files
  const paths = await globby(patterns);

  console.log(paths);
}

run().catch((err) => {
  console.error(err);
});
```

### Explanation of the Example

1. **Import Globby**: First, you import the Globby library.

2. **Define Patterns**: You define an array of glob patterns. In this case:

   - `*.js` matches all JavaScript files in the current directory.
   - `src/**/*.js` matches all JavaScript files in the `src` directory and any of its subdirectories.
   - `!**/node_modules/**` excludes any files in the `node_modules` directory.

3. **Run the Function**: You call the `globby()` function with the defined patterns. This returns a promise, which resolves to an array of matching file paths.

4. **Output the Results**: The matching paths are logged to the console.

### Advanced Usage

**Using Options with Globby**

You can also pass options to modify its behavior:

```javascript
const globby = require("globby");

async function run() {
  const options = {
    dot: true, // Include hidden files
    gitignore: true, // Respect .gitignore files
    expandDirectories: {
      extensions: ["js", "ts"], // Expand directories for .js and .ts files
    },
  };

  const paths = await globby(["src/*", "lib/**/*"], options);

  console.log(paths);
}

run().catch((err) => {
  console.error(err);
});
```

### Explanation of Advanced Example

1. **Options Object**: You create an options object to control:

   - **`dot`**: Set to `true` to include hidden files.
   - **`gitignore`**: Set to `true` to respect `.gitignore` files.
   - **`expandDirectories`**: Specifies that you want to expand directories matching certain extensions (`.js` and `.ts`).

2. **Calling Globby with Options**: You pass the options object as the second argument to the `globby()` call.

### Conclusion

Globby is a powerful yet user-friendly library that makes file matching in Node.js more accessible and efficient. Its promise-based API and support for various options make it a great choice for developers looking to work with file paths using glob patterns. By leveraging Globby, you can quickly match files and streamline file processing in your applications.

---

## Questions & Answers

### 1. **What is Globby, and how does it differ from the basic glob library?**

**Answer:**
Globby is a user-friendly wrapper around the `fast-glob` library that simplifies glob pattern matching for file paths. Unlike the basic `glob` library, Globby provides a promise-based API, sensible defaults, better handling of symlinks, and support for features like `.gitignore` files. It allows you to match multiple patterns in a single call and includes options like `dot` to match hidden files and `expandDirectories` to expand directory paths into their contents.

---

### 2. **How do you install Globby in a Node.js project?**

**Answer:**
You can install Globby using npm with the following command:

```bash
npm install globby
```

This will add Globby to your project's dependencies, allowing you to import and use it in your application.

---

### 3. **Can you provide a basic example of using Globby to match JavaScript files?**

**Answer:**
Sure! Here’s a simple example that matches all `.js` files in the current directory and subdirectories:

```javascript
const globby = require("globby");

async function run() {
  const paths = await globby(["**/*.js"]);
  console.log(paths);
}

run().catch((err) => {
  console.error(err);
});
```

In this example, `globby(['**/*.js'])` will return an array of all JavaScript files in the project directory and its subdirectories.

---

### 4. **What options can you pass to Globby, and what do they do?**

**Answer:**
Globby accepts several options to customize its behavior:

- **`dot`**: If set to `true`, includes hidden files (those starting with a dot).
- **`gitignore`**: If set to `true`, respects `.gitignore` files, excluding matched files listed there.
- **`expandDirectories`**: Allows you to expand directory paths into their contents based on specified extensions.

Example using options:

```javascript
const paths = await globby("src/**/*", { dot: true, gitignore: true });
```

This example would match all files in the `src` directory, including hidden files and respecting `.gitignore`.

---

### 5. **How can you exclude specific directories when using Globby?**

**Answer:**
You can exclude specific directories using the negation pattern `!`. For example, to exclude the `node_modules` directory, you can do the following:

```javascript
const paths = await globby(["**/*.js", "!**/node_modules/**"]);
```

This matches all `.js` files but excludes those located in the `node_modules` directory.

---

### 6. **What is the `expandDirectories` option in Globby, and how do you use it?**

**Answer:**
The `expandDirectories` option allows you to expand directory matches into their contents based on specified extensions. This is useful for automatically including files with certain extensions found within matched directories.

Example:

```javascript
const globby = require("globby");

const paths = await globby("src/**", {
  expandDirectories: {
    extensions: ["js", "ts"], // Expand directories for .js and .ts files
  },
});
```

In this example, if `src` contains subdirectories with `.js` or `.ts` files, those files will be included in the results.

---

### 7. **Can Globby handle symlinks, and how can you control this behavior?**

**Answer:**
Yes, Globby can handle symlinks. By default, it follows symlinks. However, if you want to prevent it from following them, you can set the `follow` option to `false`:

```javascript
const paths = await globby("**/*", { follow: false });
```

This will return matched paths without following any symlinked directories or files.

---

### 8. **How do you perform a synchronous file search using Globby?**

**Answer:**
Globby provides a synchronous method, `globby.sync()`, that you can use to perform file searches synchronously. This is useful in scripts where you want immediate results without dealing with promises.

Example:

```javascript
const paths = globby.sync("**/*.js");
console.log(paths);
```

This will synchronously return an array of all `.js` files matching the pattern.

---

### 9. **How does Globby handle performance with large file systems?**

**Answer:**
Globby, based on `fast-glob`, is optimized for performance, but when dealing with large file systems, you can improve performance by:

- Avoiding overly broad patterns (like `**`) where not necessary.
- Using the `ignore` option to exclude large directories (e.g., `node_modules`).
- Processing files in batches if you expect a massive number of results.

---

### 10. **Can you give an example of using Globby with multiple patterns?**

**Answer:**
Absolutely! You can match files against multiple glob patterns by passing an array of patterns to Globby. Here's an example:

```javascript
const paths = await globby(["*.js", "*.ts"]);
console.log(paths); // Matches both .js and .ts files in the current directory
```

In this example, Globby will return an array containing paths of both JavaScript and TypeScript files in the current directory.

---

### 11. **What are the advantages of using Globby over the traditional `glob` library?**

**Answer:**
The advantages of using Globby over the traditional `glob` library include:

- **Promise-based API**: Globby supports both promise and callback styles, allowing for cleaner asynchronous code with `async/await`.
- **Multiple Pattern Matching**: You can pass an array of glob patterns, which makes it easier to match different file types in a single call.
- **Improved Defaults**: Globby provides sensible defaults that reduce the need for configuration and streamline the matching process.
- **Built-in Support for `.gitignore`**: Globby automatically respects `.gitignore` files, making it more suitable for Git-based projects.
- **Advanced Features**: Globby includes features like `expandDirectories`, enabling users to easily handle directories containing specific file types.

---

### 12. **How does Globby handle errors when no files are matched?**

**Answer:**
When using Globby, if no files match the provided patterns, it does not throw an error. Instead, it simply returns an empty array. You can check the result to see if any paths were matched and handle it accordingly:

```javascript
const paths = await globby("nonexistent/*.js");

if (paths.length === 0) {
  console.log("No files matched the pattern.");
}
```

---

### 13. **Can Globby be used with TypeScript? If so, how?**

**Answer:**
Yes, Globby can be used with TypeScript. You can install its type definitions to get type support:

```bash
npm install --save-dev @types/globby
```

Then, you can import and use Globby in your TypeScript files like this:

```typescript
import globby from "globby";

async function run() {
  const paths: string[] = await globby("src/**/*.ts");
  console.log(paths);
}

run().catch((err) => {
  console.error(err);
});
```

This allows you to benefit from type checking and IntelliSense when using Globby in a TypeScript project.

---

### 14. **How can you handle complex matching scenarios with Globby?**

**Answer:**
You can handle complex matching scenarios by combining multiple glob patterns and using negation patterns. For example, to match all `.js` and `.ts` files but exclude certain directories, you can use:

```javascript
const paths = await globby([
  "**/*.js",
  "**/*.ts",
  "!**/test/**", // Exclude test directories
  "!**/node_modules/**", // Exclude node_modules
]);
```

This will return all JavaScript and TypeScript files while excluding any files found in the `test` and `node_modules` directories.

---

### 15. **Explain the use of the `nodir` option in Globby.**

**Answer:**
The `nodir` option in Globby, when set to `true`, prevents matching directories in the results. This is useful when you only want to retrieve files and not any directories. Here's an example:

```javascript
const paths = await globby("src/**", { nodir: true });
console.log(paths); // Only files will be logged, no directories
```

In this example, only files from the `src` directory and its subdirectories will be returned.

---

### 16. **Can Globby handle regular expressions for matching?**

**Answer:**
No, Globby does not support regular expressions for matching. It specifically uses glob patterns, which have a different syntax. However, you can filter the results after obtaining them from Globby using regular expressions if you need more complex matching.

Example:

```javascript
const paths = await globby("src/**/*.js");
const filteredPaths = paths.filter((path) => /test/.test(path)); // Filter using regex
console.log(filteredPaths);
```

This example retrieves all `.js` files and then filters them to only include those that contain the word "test" in their path.

---

### 17. **How do you integrate Globby with build tools like Gulp or Webpack?**

**Answer:**
You can integrate Globby with build tools like Gulp or Webpack to manage file operations efficiently. For example, in a Gulp task, you might use Globby to gather files to be processed:

```javascript
const gulp = require("gulp");
const globby = require("globby");

gulp.task("copy-js", async () => {
  const paths = await globby("src/**/*.js");
  return gulp.src(paths).pipe(gulp.dest("dist/js"));
});
```

In this example, Globby is used to find all JavaScript files in the `src` directory, which are then copied to the `dist/js` directory.

---

### 18. **What is the `gitignore` option, and how does it work in Globby?**

**Answer:**
The `gitignore` option, when set to `true`, tells Globby to respect the `.gitignore` file present in the project directory. This means any files or directories listed in the `.gitignore` will be excluded from the results.

Example:

```javascript
const paths = await globby("**/*", { gitignore: true });
console.log(paths); // Results will not include files ignored by .gitignore
```

This feature is particularly useful for avoiding the inclusion of build artifacts, dependencies, or other files that shouldn't be part of your source code.

---

### 19. **How do you use Globby with asynchronous file operations?**

**Answer:**
You can use Globby with asynchronous file operations by using the `async/await` syntax. Here’s a simple example where you read and log the contents of matched files:

```javascript
const globby = require("globby");
const fs = require("fs").promises;

async function logFileContents() {
  const paths = await globby("src/**/*.js");
  for (const path of paths) {
    const content = await fs.readFile(path, "utf-8");
    console.log(`Contents of ${path}:`);
    console.log(content);
  }
}

logFileContents().catch((err) => {
  console.error(err);
});
```

In this example, Globby matches all `.js` files, and the contents of each matched file are read asynchronously and logged.

---

### 20. **What performance considerations should you keep in mind when using Globby?**

**Answer:**
When using Globby, consider the following performance considerations:

- **Use specific patterns**: Avoid overly broad patterns (like `**`) to reduce the number of files being processed.
- **Leverage negation patterns**: Use negation to exclude large directories (e.g., `node_modules`) to improve matching speed.
- **Batch processing**: If dealing with a massive number of files, consider processing them in smaller batches rather than all at once.
- **Limit recursive depth**: Use patterns that limit the recursion depth if you don't need to search deeply nested directories.

By keeping these considerations in mind, you can optimize the performance of file matching and processing in your applications using Globby.
