# Working with Files

Working with files in Node.js is commonly done using the built-in `fs` (file system) module. The `fs` module provides both synchronous and asynchronous methods to interact with the file system, allowing you to read, write, update, and delete files.

Here’s a breakdown of some of the most commonly used file operations:

### 1. **Reading Files**

You can read the contents of a file asynchronously or synchronously.

**Asynchronous method:**

```javascript
const fs = require("fs");

fs.readFile("file.txt", "utf8", (err, data) => {
  if (err) {
    console.error(err);
    return;
  }
  console.log(data);
});
```

**Synchronous method:**

```javascript
const fs = require("fs");

try {
  const data = fs.readFileSync("file.txt", "utf8");
  console.log(data);
} catch (err) {
  console.error(err);
}
```

### 2. **Writing to Files**

You can write content to a file. If the file doesn't exist, it will be created. Otherwise, it will be overwritten.

**Asynchronous method:**

```javascript
fs.writeFile("file.txt", "Hello, world!", (err) => {
  if (err) {
    console.error(err);
  } else {
    console.log("File has been written.");
  }
});
```

**Synchronous method:**

```javascript
try {
  fs.writeFileSync("file.txt", "Hello, world!");
  console.log("File has been written.");
} catch (err) {
  console.error(err);
}
```

### 3. **Appending to Files**

You can append content to an existing file using the `appendFile` function.

**Asynchronous method:**

```javascript
fs.appendFile("file.txt", "\nAppended text.", (err) => {
  if (err) {
    console.error(err);
  } else {
    console.log("Text has been appended.");
  }
});
```

**Synchronous method:**

```javascript
try {
  fs.appendFileSync("file.txt", "\nAppended text.");
  console.log("Text has been appended.");
} catch (err) {
  console.error(err);
}
```

### 4. **Deleting Files**

To delete a file, use the `unlink` method.

**Asynchronous method:**

```javascript
fs.unlink("file.txt", (err) => {
  if (err) {
    console.error(err);
  } else {
    console.log("File deleted.");
  }
});
```

**Synchronous method:**

```javascript
try {
  fs.unlinkSync("file.txt");
  console.log("File deleted.");
} catch (err) {
  console.error(err);
}
```

### 5. **Checking if a File Exists**

Node.js deprecated `fs.exists`, but you can use `fs.access` to check if a file exists:

```javascript
fs.access("file.txt", fs.constants.F_OK, (err) => {
  if (err) {
    console.error("File does not exist");
  } else {
    console.log("File exists");
  }
});
```

### 6. **Working with Directories**

To create a directory, you can use `fs.mkdir`:

**Asynchronous method:**

```javascript
fs.mkdir("mydir", { recursive: true }, (err) => {
  if (err) {
    console.error(err);
  } else {
    console.log("Directory created.");
  }
});
```

**Synchronous method:**

```javascript
try {
  fs.mkdirSync("mydir", { recursive: true });
  console.log("Directory created.");
} catch (err) {
  console.error(err);
}
```

To remove a directory:

**Asynchronous method:**

```javascript
fs.rmdir("mydir", (err) => {
  if (err) {
    console.error(err);
  } else {
    console.log("Directory removed.");
  }
});
```

### 7. **Streaming Files**

For larger files, using streams is more efficient than reading the entire file into memory.

**Reading a file as a stream:**

```javascript
const readStream = fs.createReadStream("file.txt", "utf8");

readStream.on("data", (chunk) => {
  console.log("Received chunk:", chunk);
});

readStream.on("end", () => {
  console.log("Finished reading file.");
});
```

**Writing to a file using a stream:**

```javascript
const writeStream = fs.createWriteStream("file.txt");

writeStream.write("Hello, ");
writeStream.write("world!");
writeStream.end();

writeStream.on("finish", () => {
  console.log("Finished writing to file.");
});
```

### **Basic Questions**

1. **What is the `fs` module in Node.js?**

   - The `fs` module is a built-in module in Node.js that provides functions for interacting with the file system, such as reading, writing, deleting, and manipulating files and directories.

2. **How do you read a file synchronously in Node.js?**

   ```js
   const fs = require("fs");
   const data = fs.readFileSync("file.txt", "utf8");
   console.log(data);
   ```

   - This code reads the content of `file.txt` synchronously, meaning it blocks the execution until the file is completely read.

3. **How do you read a file asynchronously in Node.js?**

   ```js
   const fs = require("fs");
   fs.readFile("file.txt", "utf8", (err, data) => {
     if (err) throw err;
     console.log(data);
   });
   ```

   - This method reads the file asynchronously without blocking other operations.

4. **What is the difference between `fs.readFileSync()` and `fs.readFile()`?**

   - `fs.readFileSync()` reads the file synchronously, blocking the event loop until the operation is completed.
   - `fs.readFile()` is asynchronous and non-blocking, allowing the event loop to handle other operations while the file is being read.

5. **How do you write to a file in Node.js?**

   ```js
   const fs = require("fs");
   fs.writeFile("file.txt", "Hello World", (err) => {
     if (err) throw err;
     console.log("File written successfully");
   });
   ```

6. **How do you append data to an existing file in Node.js?**

   ```js
   const fs = require("fs");
   fs.appendFile("file.txt", " Appending this text.", (err) => {
     if (err) throw err;
     console.log("Data appended");
   });
   ```

7. **How do you delete a file in Node.js?**
   ```js
   const fs = require("fs");
   fs.unlink("file.txt", (err) => {
     if (err) throw err;
     console.log("File deleted");
   });
   ```

### **Intermediate Questions**

8. **How do you check if a file or directory exists in Node.js?**

   - Using `fs.existsSync()` (synchronous):

     ```js
     const fs = require("fs");
     if (fs.existsSync("file.txt")) {
       console.log("File exists");
     } else {
       console.log("File does not exist");
     }
     ```

   - Using `fs.access()` (asynchronous):
     ```js
     fs.access("file.txt", fs.constants.F_OK, (err) => {
       console.log(err ? "File does not exist" : "File exists");
     });
     ```

9. **How do you create a directory in Node.js?**

   ```js
   const fs = require("fs");
   fs.mkdir("new-folder", (err) => {
     if (err) throw err;
     console.log("Directory created");
   });
   ```

10. **How do you read the contents of a directory in Node.js?**

    ```js
    const fs = require("fs");
    fs.readdir("./", (err, files) => {
      if (err) throw err;
      console.log(files);
    });
    ```

11. **How do you rename a file in Node.js?**

    ```js
    const fs = require("fs");
    fs.rename("oldName.txt", "newName.txt", (err) => {
      if (err) throw err;
      console.log("File renamed");
    });
    ```

12. **How do you get the stats of a file or directory?**
    ```js
    const fs = require("fs");
    fs.stat("file.txt", (err, stats) => {
      if (err) throw err;
      console.log(stats);
    });
    ```

### **Advanced Questions**

13. **What is the purpose of `fs.watch()` in Node.js? How does it work?**

    - `fs.watch()` is used to watch for changes in files or directories. It listens for events like file changes, renaming, or deletion and triggers a callback when an event occurs.

    ```js
    const fs = require("fs");
    fs.watch("file.txt", (eventType, filename) => {
      console.log(`Event: ${eventType}`);
      console.log(`File changed: ${filename}`);
    });
    ```

14. **What is the difference between `fs.watch()` and `fs.watchFile()`?**

    - `fs.watch()` uses operating system file change notifications and is faster, but less reliable for heavy usage.
    - `fs.watchFile()` uses polling to detect file changes, making it more reliable for frequently changed files but with more resource usage.

15. **How do you create a readable or writable stream using the `fs` module?**

    - For readable streams:
      ```js
      const fs = require("fs");
      const readStream = fs.createReadStream("file.txt", "utf8");
      readStream.on("data", (chunk) => {
        console.log("Data:", chunk);
      });
      ```
    - For writable streams:
      ```js
      const fs = require("fs");
      const writeStream = fs.createWriteStream("file.txt");
      writeStream.write("Hello, this is a stream write operation.");
      writeStream.end();
      ```

16. **How do you copy a file in Node.js?**

    ```js
    const fs = require("fs");
    fs.copyFile("source.txt", "destination.txt", (err) => {
      if (err) throw err;
      console.log("File copied");
    });
    ```

17. **How do you handle large files in Node.js to prevent memory overflow?**

    - You can use streams to handle large files efficiently:
      ```js
      const fs = require("fs");
      const readStream = fs.createReadStream("largeFile.txt");
      const writeStream = fs.createWriteStream("outputFile.txt");
      readStream.pipe(writeStream);
      ```

18. **How can you ensure atomic file operations in Node.js?**

    - For atomic writes, you can use temporary files and rename them after writing the content:
      ```js
      const fs = require("fs");
      const tmpFile = "file.tmp";
      fs.writeFile(tmpFile, "Data", (err) => {
        if (err) throw err;
        fs.rename(tmpFile, "file.txt", (err) => {
          if (err) throw err;
          console.log("File written atomically");
        });
      });
      ```

19. **How do you manage file permissions in Node.js?**

    - You can change file permissions using `fs.chmod()`:
      ```js
      const fs = require("fs");
      fs.chmod("file.txt", 0o755, (err) => {
        if (err) throw err;
        console.log("Permissions changed");
      });
      ```

20. **Explain how you can handle file system errors gracefully in a production environment.**

    - Always check for error codes and handle them accordingly:
      ```js
      const fs = require("fs");
      fs.readFile("file.txt", (err, data) => {
        if (err) {
          if (err.code === "ENOENT") {
            console.log("File not found");
          } else {
            throw err;
          }
        } else {
          console.log(data);
        }
      });
      ```

21. **How do you recursively create a directory in Node.js?**

- Starting from Node.js 10.x, you can use the `recursive` option in `fs.mkdir()` to create nested directories:
  ```js
  const fs = require("fs");
  fs.mkdir("dir1/dir2/dir3", { recursive: true }, (err) => {
    if (err) throw err;
    console.log("Directories created recursively");
  });
  ```
- This will create the entire path if it doesn’t exist.

22. **How do you recursively delete a directory in Node.js?**

- You can use the `fs.rmdir()` method with the `recursive` option (from Node.js v12.x and later):
  ```js
  const fs = require("fs");
  fs.rmdir("dir", { recursive: true }, (err) => {
    if (err) throw err;
    console.log("Directory deleted recursively");
  });
  ```

23. **How can you copy an entire directory in Node.js?**

- Node.js does not have a built-in method to copy directories. You can do this by reading the directory contents recursively, creating directories, and copying files. Here’s an example:

  ```js
  const fs = require("fs");
  const path = require("path");

  function copyDirectory(src, dest) {
    fs.mkdirSync(dest, { recursive: true });
    const entries = fs.readdirSync(src, { withFileTypes: true });
    for (let entry of entries) {
      const srcPath = path.join(src, entry.name);
      const destPath = path.join(dest, entry.name);

      entry.isDirectory()
        ? copyDirectory(srcPath, destPath)
        : fs.copyFileSync(srcPath, destPath);
    }
  }

  copyDirectory("sourceDir", "destinationDir");
  ```

24. **How can you monitor a file for changes in its content?**

- You can use `fs.watchFile()` to monitor a file for changes:
  ```js
  const fs = require("fs");
  fs.watchFile("file.txt", (curr, prev) => {
    console.log(`File changed from ${prev.mtime} to ${curr.mtime}`);
  });
  ```
- `curr` and `prev` are `fs.Stats` objects representing the current and previous states of the file.

25. **What is the difference between `fs.watch()` and `fs.watchFile()` in terms of performance?**

- `fs.watch()` is event-driven, notifying changes based on file system events, and is faster and less resource-intensive.
- `fs.watchFile()` uses polling to detect file changes, which can be less efficient for large numbers of files but is more reliable on certain file systems (like NFS) where `fs.watch()` may not work well.

26. **What are `fs.constants` and how do you use them?**

- `fs.constants` is an object containing commonly used constants for file operations such as read, write, append, and permission settings.
  ```js
  const fs = require("fs");
  fs.access("file.txt", fs.constants.R_OK | fs.constants.W_OK, (err) => {
    if (err) {
      console.log("No read/write permissions");
    } else {
      console.log("Read and write permissions available");
    }
  });
  ```
- You can use these constants to check for permissions, modes, and file states.

27. **How do you handle errors when performing file operations in Node.js?**

- File system errors can be handled by checking the `err.code` and responding accordingly. Common error codes include:
  - `ENOENT` (No such file or directory)
  - `EACCES` (Permission denied)
  - `EEXIST` (File or directory already exists)
  ```js
  const fs = require("fs");
  fs.readFile("nonexistent.txt", (err, data) => {
    if (err) {
      if (err.code === "ENOENT") {
        console.log("File does not exist");
      } else {
        throw err;
      }
    } else {
      console.log(data.toString());
    }
  });
  ```

28. **How can you truncate a file in Node.js?**

- You can use `fs.truncate()` to shrink or expand the file to a specified length. If the file is larger than the specified length, it will be truncated; otherwise, it will be padded with null bytes.
  ```js
  const fs = require("fs");
  fs.truncate("file.txt", 100, (err) => {
    if (err) throw err;
    console.log("File truncated to 100 bytes");
  });
  ```

29. **How do you work with symbolic links in Node.js?**

- You can create symbolic links using `fs.symlink()` and read their destination using `fs.readlink()`:

  ```js
  const fs = require("fs");
  fs.symlink("target.txt", "symlink.txt", (err) => {
    if (err) throw err;
    console.log("Symbolic link created");
  });

  fs.readlink("symlink.txt", (err, linkString) => {
    if (err) throw err;
    console.log(`Symbolic link points to: ${linkString}`);
  });
  ```

30. **How can you read a large file in chunks without loading it entirely into memory?**

    - You can use the `fs.createReadStream()` method to read the file as a stream and process it in chunks:

      ```js
      const fs = require("fs");
      const readStream = fs.createReadStream("largeFile.txt", {
        encoding: "utf8",
        highWaterMark: 16 * 1024,
      });

      readStream.on("data", (chunk) => {
        console.log(`Received ${chunk.length} bytes of data.`);
      });

      readStream.on("end", () => {
        console.log("Finished reading file.");
      });
      ```

31. **How can you efficiently move (rename) a file in Node.js?**

    - You can use the `fs.rename()` function, which essentially moves the file by changing its path:
      ```js
      const fs = require("fs");
      fs.rename("oldPath/file.txt", "newPath/file.txt", (err) => {
        if (err) throw err;
        console.log("File moved successfully");
      });
      ```

32. **How can you monitor multiple files or directories for changes in Node.js?**

    - You can monitor multiple files using `fs.watch()` or `fs.watchFile()` by calling them for each file or directory.
    - Example for watching multiple files:

      ```js
      const fs = require("fs");
      const filesToWatch = ["file1.txt", "file2.txt"];

      filesToWatch.forEach((file) => {
        fs.watch(file, (eventType, filename) => {
          console.log(`File ${filename} had a ${eventType} event.`);
        });
      });
      ```

33. **How do you change the owner and group of a file in Node.js?**

    - You can use `fs.chown()` to change the ownership of a file:
      ```js
      const fs = require("fs");
      fs.chown("file.txt", 1000, 1000, (err) => {
        if (err) throw err;
        console.log("Owner and group changed");
      });
      ```

34. **How do you work with file descriptors in Node.js?**

    - File descriptors can be obtained using `fs.open()`. You can read, write, or manipulate files with this descriptor.
    - Example of reading a file using a descriptor:
      ```js
      const fs = require("fs");
      fs.open("file.txt", "r", (err, fd) => {
        if (err) throw err;
        const buffer = Buffer.alloc(1024);
        fs.read(fd, buffer, 0, buffer.length, 0, (err, bytesRead) => {
          if (err) throw err;
          console.log(buffer.slice(0, bytesRead).toString());
          fs.close(fd, (err) => {
            if (err) throw err;
          });
        });
      });
      ```

35. **What is the difference between `fs.fstat()` and `fs.stat()`?**

    - `fs.stat()` takes a file path and returns the stats (metadata) of the file.
    - `fs.fstat()` takes a file descriptor (an integer returned by `fs.open()`) and returns the stats of the file associated with that descriptor.

36. **How can you copy a file asynchronously and handle large files efficiently in Node.js?**

    - You can use streams to copy files efficiently without loading the entire file into memory:

      ```js
      const fs = require("fs");
      const readStream = fs.createReadStream("source.txt");
      const writeStream = fs.createWriteStream("destination.txt");
      readStream.pipe(writeStream);

      readStream.on("end", () => {
        console.log("File copied successfully");
      });
      ```
