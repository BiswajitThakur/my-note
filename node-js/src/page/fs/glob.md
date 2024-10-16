# [`glob`](https://www.npmjs.com/package/glob)

### What is Glob?

A **glob** is a pattern that is used to match filenames and paths, often involving wildcard characters. It is a shorthand way of selecting groups of files in a directory structure based on their names, extensions, or patterns. This is commonly used in file systems and programming to deal with files dynamically, without needing to manually list each one.

In Node.js and many other environments (like Unix shells), globbing is a way to search for files that match a specific pattern. The most common characters used in glob patterns are `*`, `?`, and `[]`.

### Common Wildcards and Patterns in Glob

1. **Asterisk (`*`)**: Matches any sequence of characters (including none).

   - Example:
     - Pattern: `*.js`
     - Matches: `app.js`, `index.js`, but not `readme.md`.

2. **Question Mark (`?`)**: Matches a single character.

   - Example:
     - Pattern: `file?.txt`
     - Matches: `file1.txt`, `file2.txt`, but not `file.txt` or `file11.txt`.

3. **Square Brackets (`[]`)**: Matches any one of the enclosed characters or ranges.

   - Example:
     - Pattern: `file[12].txt`
     - Matches: `file1.txt`, `file2.txt`, but not `file3.txt`.

4. **Brace Expansion (`{}`)**: Matches any pattern separated by commas inside the braces.

   - Example:
     - Pattern: `file.{js,txt}`
     - Matches: `file.js`, `file.txt`, but not `file.md`.

5. **Double Asterisk (`**`)\*\*: Matches directories recursively.

   - Example:
     - Pattern: `**/*.js`
     - Matches: `app.js`, `src/index.js`, `lib/utils/parser.js` (matches `.js` files in all subdirectories).

6. **Negation (`!`)**: Excludes files that match the pattern.
   - Example:
     - Pattern: `!*.min.js`
     - Matches: All files except those that end with `.min.js`.

---

### Examples of Glob Patterns

1. **`*.txt`**: Matches all `.txt` files in the current directory.

   - Example:
     - Matches: `readme.txt`, `notes.txt`
     - Does not match: `index.html`, `script.js`

2. **`src/**/\*.js`**: Matches all `.js`files in the`src/` directory and any of its subdirectories.

   - Example:
     - Matches: `src/index.js`, `src/utils/helper.js`, `src/lib/someScript.js`

3. **`[abc]*`**: Matches files starting with `a`, `b`, or `c`.

   - Example:
     - Matches: `apple.txt`, `banana.js`, `cat.css`
     - Does not match: `dog.png`

4. **`!(example).js`**: Matches any `.js` file that is not named `example.js`.
   - Example:
     - Matches: `app.js`, `main.js`
     - Does not match: `example.js`

---

### Use of Glob in Node.js

In Node.js, you can use the `glob` module to easily work with file system patterns. The `glob` module allows you to specify patterns and get back an array of filenames that match the pattern.

To install the `glob` module:

```bash
npm install glob
```

Example usage:

```js
const glob = require("glob");

// Find all JavaScript files in the current directory
glob("*.js", function (err, files) {
  if (err) throw err;
  console.log(files); // Array of .js files
});
```

Example for recursive search:

```js
glob("**/*.js", function (err, files) {
  if (err) throw err;
  console.log(files); // Array of .js files in all subdirectories
});
```

### Advanced Usage of Glob in Node.js

1. **Synchronous Search:**
   You can also run the search synchronously by using `glob.sync()`.

   Example:

   ```js
   const files = glob.sync("*.js");
   console.log(files);
   ```

2. **Options with Glob:**
   The `glob` function accepts options to control the behavior of pattern matching.

   Example with options:

   ```js
   glob("**/*.js", { ignore: "node_modules/**" }, function (err, files) {
     if (err) throw err;
     console.log(files); // Matches .js files but ignores node_modules directory
   });
   ```

3. **Real-World Example:**
   If you want to get all `.js` files except for those inside a `test/` folder:
   ```js
   glob("**/*.js", { ignore: "test/**" }, function (err, files) {
     if (err) throw err;
     console.log(files);
   });
   ```

---

### Why Use Glob?

- **Efficiency**: Instead of manually specifying every file path, globbing allows you to define patterns that can match a wide range of files.
- **Convenience**: Many tasks, such as building projects, running tests, or processing files, require selecting files dynamically. Glob is a simple and powerful way to do this.
- **Cross-Platform Compatibility**: It works consistently across different platforms, making it suitable for Node.js applications that might run in various environments.

---

### Alternatives to Glob in Node.js

- **`fs` module with custom filtering**: You can achieve similar functionality using the `fs` module by recursively reading directories and filtering based on file extensions or names, but glob is more concise and easier to use.
- **Third-party libraries**: Some libraries, like `fast-glob`, offer similar functionality to `glob` with improved performance and additional features.

### Conclusion

Glob patterns are widely used in file systems and tools to match files based on specific patterns. They simplify working with large sets of files, especially when you need to operate on multiple files dynamically. In Node.js, the `glob` module provides an easy and efficient way to work with file patterns for file discovery, and it is highly flexible and configurable to suit different project needs.

## Questions & Answers

### 1. **What is a glob pattern?**

**Answer:**
A **glob pattern** is a string used to match file or directory names based on wildcards or specific characters. It is commonly used for file matching and selecting files dynamically in file systems. For example, `*.js` matches all files with the `.js` extension in a directory.

---

### 2. **What are the common wildcard characters used in glob patterns?**

**Answer:**

- **Asterisk (`*`)**: Matches any sequence of characters, including none.
  - Example: `*.js` matches `app.js`, `index.js`.
- **Question mark (`?`)**: Matches a single character.

  - Example: `file?.txt` matches `file1.txt`, `file2.txt`, but not `file.txt`.

- **Square brackets (`[]`)**: Matches any one character inside the brackets.

  - Example: `file[12].txt` matches `file1.txt`, `file2.txt`.

- **Brace expansion (`{}`)**: Matches multiple patterns separated by commas.

  - Example: `file.{js,txt}` matches `file.js`, `file.txt`.

- **Double asterisk (`**`)\*\*: Recursively matches directories.
  - Example: `src/**/*.js` matches `.js` files in any subdirectory of `src/`.

---

### 3. **What is the difference between `*` and `**` in glob patterns?\*\*

**Answer:**

- **`*`**: Matches any sequence of characters, except for directory separators (`/` on Unix or `\` on Windows).

  - Example: `*.js` matches `file.js` in the current directory.

- **`**`\*\*: Matches any number of directories recursively, along with file names.
  - Example: `src/**/*.js` matches all `.js` files in `src/` and any subdirectories, such as `src/utils/helper.js`.

---

### 4. **How would you exclude specific files or directories in a glob pattern?**

**Answer:**
You can exclude files or directories using a negation pattern (`!`). For example, to match all `.js` files except for those in the `test/` directory, you can use:

```js
const glob = require("glob");

glob("**/*.js", { ignore: "test/**" }, function (err, files) {
  if (err) throw err;
  console.log(files); // This will exclude .js files in the 'test/' directory
});
```

Alternatively, you can also prepend `!` to a glob pattern:

```js
glob(["**/*.js", "!test/**/*.js"], function (err, files) {
  if (err) throw err;
  console.log(files);
});
```

---

### 5. **How does the `glob` module in Node.js work, and what is it used for?**

**Answer:**
The `glob` module in Node.js allows you to search for files and directories that match specific patterns, similar to how wildcards are used in Unix shell commands. It simplifies selecting files by allowing you to define patterns instead of hardcoding filenames.

Example:

```js
const glob = require("glob");

// Match all .js files
glob("*.js", function (err, files) {
  if (err) throw err;
  console.log(files); // Output array of matched .js files
});
```

The `glob` module is used for tasks like file system searching, loading configuration files, and processing multiple files based on patterns.

---

### 6. **What is the difference between `glob()` and `glob.sync()` in Node.js?**

**Answer:**

- **`glob()`**: Asynchronously searches for files that match the given pattern and returns the result via a callback.

  - Example:
    ```js
    glob("*.js", function (err, files) {
      if (err) throw err;
      console.log(files); // Asynchronous result
    });
    ```

- **`glob.sync()`**: Performs the search synchronously, returning the result directly.
  - Example:
    ```js
    const files = glob.sync("*.js");
    console.log(files); // Synchronous result
    ```

The synchronous version is typically used when you need the result immediately and don’t want to use callbacks or promises.

---

### 7. **How can you match all JavaScript files inside a directory and its subdirectories using glob?**

**Answer:**
To match all JavaScript files in a directory and its subdirectories, use the double asterisk (`**`) for recursive matching.

Example:

```js
glob("**/*.js", function (err, files) {
  if (err) throw err;
  console.log(files); // Matches all .js files in all subdirectories
});
```

This pattern matches `.js` files in the current directory and all nested directories.

---

### 8. **How can brace expansion be used in glob patterns?**

**Answer:**
Brace expansion allows matching multiple patterns separated by commas. It’s used to match files with different extensions or similar naming structures.

Example:

```js
glob("file.{js,txt}", function (err, files) {
  if (err) throw err;
  console.log(files); // Matches file.js and file.txt
});
```

This pattern will match both `file.js` and `file.txt`.

---

### 9. **What are some common use cases for the glob pattern?**

**Answer:**

- **Loading all files of a certain type**: For example, matching all `.js` or `.css` files in a directory.
- **Batch processing**: Processing or compiling a set of files that match a specific pattern (e.g., minifying all `.js` files).
- **Finding configuration files**: Loading configuration files like `.json` or `.env` based on a pattern.
- **Recursive directory search**: Searching for files in nested directories.

---

### 10. **What are the limitations of the `glob` module in Node.js?**

**Answer:**

- **Performance**: When dealing with very large file systems or many nested directories, `glob` can become slow. Tools like `fast-glob` can offer better performance in such cases.
- **No native streaming**: The `glob` module doesn’t stream files. For extremely large sets of files, the result is returned all at once, which may use a lot of memory.
- **Not as powerful as regular expressions**: While `glob` is flexible, it lacks the full power of regular expressions for more complex matching.

---

### 11. **How do you combine multiple glob patterns?**

**Answer:**
You can combine multiple glob patterns using an array. For example, if you want to match both `.js` and `.css` files, you can use:

```js
glob(["**/*.js", "**/*.css"], function (err, files) {
  if (err) throw err;
  console.log(files); // Matches both .js and .css files
});
```

This pattern matches both `.js` and `.css` files in all directories.

---

### 12. **What is the use of the `ignore` option in the `glob` module?**

**Answer:**
The `ignore` option in the `glob` module is used to exclude specific files or directories from the search. It can be useful when you want to match a broad range of files but exclude certain subdirectories like `node_modules` or `test`.

Example:

```js
glob("**/*.js", { ignore: "node_modules/**" }, function (err, files) {
  if (err) throw err;
  console.log(files); // All .js files, except in the node_modules directory
});
```

---

### 13. **How does glob handle absolute paths vs relative paths?**

**Answer:**
Glob patterns typically use relative paths to the current working directory. However, you can use absolute paths directly in glob patterns.

Example with an absolute path:

```js
glob("/Users/username/project/**/*.js", function (err, files) {
  if (err) throw err;
  console.log(files); // Matches all .js files in the specified directory
});
```

### 14. **How does the glob module handle file system links (symlinks)?**

**Answer:**
By default, the `glob` module follows symbolic links (symlinks) when searching through directories. However, you can disable this behavior by setting the `follow` option to `false`.

Example:

```js
glob("**/*.js", { follow: false }, function (err, files) {
  if (err) throw err;
  console.log(files); // Won't follow symbolic links
});
```

Here are more **glob** pattern interview questions and answers to further explore the topic:

### 15. **How does the glob pattern handle hidden files and directories?**

**Answer:**
In most systems, files or directories that start with a dot (`.`) are considered hidden. By default, glob patterns do not match hidden files or directories unless you explicitly include the dot in the pattern.

Example:

- Pattern: `*.js` will **not** match `.hidden.js`.
- Pattern: `.*.js` will match hidden files like `.hidden.js`.

To match both hidden and non-hidden files, you can combine patterns:

```js
glob("{.*,}*.js", function (err, files) {
  console.log(files); // Matches both hidden and non-hidden .js files
});
```

---

### 16. **What is brace expansion in glob, and how can you use it effectively?**

**Answer:**
Brace expansion is a feature that allows you to specify multiple patterns within curly braces `{}` separated by commas, which is especially useful when you want to match different file extensions or similar patterns.

Example:

```js
glob("file.{js,txt,md}", function (err, files) {
  console.log(files); // Matches file.js, file.txt, file.md
});
```

This is useful for scenarios like processing multiple file types in a single operation.

---

### 17. **Can glob patterns handle absolute paths, and how does it affect pattern matching?**

**Answer:**
Yes, glob patterns can handle both relative and absolute paths. When using absolute paths, the glob will attempt to match from the root of the file system.

Example:

```js
glob("/Users/username/project/**/*.js", function (err, files) {
  console.log(files); // Matches all .js files in /Users/username/project directory and subdirectories
});
```

Using absolute paths can be useful in scripts where you need to match files across various parts of the filesystem, but it is generally best practice to work with relative paths in cross-platform environments.

---

### 18. **How would you match all files except those in specific directories using glob?**

**Answer:**
You can exclude specific directories by using the `ignore` option or by negating patterns with a `!` in front of the pattern.

Example using `ignore` option:

```js
glob("**/*.js", { ignore: "node_modules/**" }, function (err, files) {
  console.log(files); // Matches all .js files except those in node_modules directory
});
```

Example using negation:

```js
glob(["**/*.js", "!node_modules/**"], function (err, files) {
  console.log(files); // Same as above
});
```

This is useful when you want to avoid processing files in certain directories, like `node_modules` or `test`.

---

### 19. **What is the difference between `globstar` (`**`) and `\*` in glob?\*\*

**Answer:**

- **`*`**: Matches zero or more characters, but only within the current directory (does not cross directories).

  - Example: `*.js` matches files like `file.js`, but not files in subdirectories like `src/index.js`.

- **`**`**: The **globstar** (`\*\*`) allows matching across directory boundaries, recursively traversing directories.
  - Example: `src/**/*.js` matches `src/index.js`, `src/utils/helper.js`, and any `.js` files in nested directories under `src`.

Use `**` when you need to recursively match files across directories, and `*` when you are only matching within a single directory level.

---

### 20. **How do you handle errors in the `glob` module in Node.js?**

**Answer:**
Errors in the `glob` module are handled using the callback function provided to `glob()`. If an error occurs (e.g., permission issues or invalid patterns), the error is passed as the first argument to the callback function.

Example:

```js
glob("*.js", function (err, files) {
  if (err) {
    console.error("Error occurred:", err);
    return;
  }
  console.log("Matched files:", files);
});
```

In case of errors, you should always check if `err` is not `null` and handle it appropriately, such as logging the error or providing fallback behavior.

---

### 21. **What is `nodir` option in glob and when should you use it?**

**Answer:**
The `nodir` option is used to exclude directories from the results, ensuring that only files are returned.

Example:

```js
glob("**/*", { nodir: true }, function (err, files) {
  console.log(files); // Only files, no directories
});
```

You should use this option when you only want to work with files and not directories, such as when you are processing files like `.js` or `.txt` but don't want directories in your result.

---

### 22. **What is the `dot` option in glob, and how does it affect file matching?**

**Answer:**
The `dot` option in glob allows you to match hidden files (files that begin with a dot `.`). By default, glob ignores hidden files unless this option is enabled.

Example:

```js
glob("**/*", { dot: true }, function (err, files) {
  console.log(files); // Matches hidden files like .gitignore
});
```

Use the `dot` option when you want to include hidden files (e.g., `.env`, `.gitignore`) in your matches.

---

### 23. **How can you perform a synchronous file search using glob?**

**Answer:**
In addition to the asynchronous `glob()` function, you can use the `glob.sync()` method to perform synchronous searches. This can be useful when you need the result immediately and don’t want to use a callback or promise.

Example:

```js
const files = glob.sync("*.js");
console.log(files); // Synchronous result
```

Synchronous glob searches are generally used in scripts or places where you don't want to deal with asynchronous code flow.

---

### 24. **What are the performance considerations when using glob for large file systems?**

**Answer:**
When working with very large file systems, glob can become slow because it recursively searches through directories to match files. Some performance considerations include:

- **Limiting recursion**: Avoid using `**` unnecessarily, as it searches across many directories.
- **Use the `ignore` option**: Exclude large directories like `node_modules` to improve performance.
- **Use `nodir` or `dot`**: If you only need files or don't need hidden files, use these options to minimize the search scope.
- **Use alternative libraries**: For very large file systems, libraries like `fast-glob` offer better performance than the default `glob` module.

---

### 25. **How do you match files with specific extensions using glob?**

**Answer:**
To match files with specific extensions, use the `*` wildcard combined with the desired extension. For example, to match all `.js` files, use:

```js
glob("*.js", function (err, files) {
  console.log(files); // Matches all .js files
});
```

You can also match multiple extensions using brace expansion:

```js
glob("*.{js,ts}", function (err, files) {
  console.log(files); // Matches all .js and .ts files
});
```

This allows you to select files based on their extension without listing each file explicitly.

---

### 26. **How does glob handle symlinks, and how can you prevent it from following them?**

**Answer:**
By default, glob follows symbolic links (symlinks) when searching directories. To prevent glob from following symlinks, you can use the `follow` option and set it to `false`.

Example:

```js
glob("**/*.js", { follow: false }, function (err, files) {
  console.log(files); // Will not follow symlinks
});
```

This is useful when you don’t want to traverse through symlinked directories or files, which could lead to unexpected results or performance issues.

---

### 27. **How can you debug glob patterns to ensure they are matching the right files?**

**Answer:**
To debug glob patterns, you can print out the matched files and review them for correctness. If files are missing or extra files are included, adjust the pattern.

Example:

```js
glob("*.js", function (err, files) {
  if (err) {
    console.error("Error occurred:", err);
    return;
  }
  console.log("Matched files:", files); // Review matched files to ensure pattern is correct
});
```

Additionally, try breaking the pattern into smaller parts and testing them individually to isolate any issues.

---

### 28. **How do you handle large file sets with glob to avoid memory overload?**

**Answer:**
For very large file sets, you can:

- **Avoid recursive patterns (`**`) where unnecessary\*\*.
- **Use the `ignore` option** to skip directories with many files (e.g., `node_modules`).
- **Process files in batches** if the file set is large, to avoid loading all results into memory at once.
- **Switch to streaming-based libraries** if the file set is massive and glob isn’t sufficient.
