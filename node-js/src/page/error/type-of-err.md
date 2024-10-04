# Types of Errors

Error handling in Node.js can be categorized based on the nature of the error. The most common error types are:

1. System Errors
1. User-Specified Errors
1. Assertion Errors
1. JavaScript Errors

## System Errors

System errors occur when Node.js encounters issues interacting with the operating system or external resources, such as file system errors, network errors, or permission issues. These errors are often triggered by low-level system calls.

Below are the system errors commonly encountered when writing a Node.js program

- EACCES - Permission denied
- EADDRINUSE - Address already in use
- ECONNRESET - Connection reset by peer
- EEXIST - File exists
- EISDIR - Is a directory
- EMFILE - Too many open files in system
- ENOENT - No such file or directory
- ENOTDIR - Not a directory
- ENOTEMPTY - Directory not empty
- ENOTFOUND - DNS lookup failed
- EPERM - Operation not permitted
- EPIPE - Broken Pipe
- ETIMEDOUT - Operation timed out

**Example: File System Error**

```js
const fs = require("fs");

fs.readFile("nonexistent-file.txt", "utf8", (err, data) => {
  if (err) {
    console.error("System Error:", err.code); // 'ENOENT' for file not found
    console.error("Error Message:", err.message); // Error description
    console.error("Error Stack:", err.stack); // Stack trace
    return;
  }
  console.log(data);
});
```

**In this example:**

- `ENOENT` is a system-level error code that signifies "Error NO ENTry" (i.e., file not found).
- Such errors occur due to external conditions beyond the control of the program, like file permissions, missing files, or unavailable network resources.

**Common system errors:**

- `ENOENT`: File or directory not found.
- `EACCES`: Permission denied.
- `ECONNREFUSED`: Connection refused.
- `EADDRINUSE`: Address already in use.
