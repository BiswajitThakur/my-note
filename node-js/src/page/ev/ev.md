# Environment Variables

**Environment Variables** are key-value pairs that can affect the behavior of processes running on a computer. They are often used to configure settings or provide information to applications at runtime without hardcoding those values into the code.

### Key Points:

1. **Usage**:

   - Commonly used to store sensitive information such as API keys, database credentials, and configuration settings.
   - They help in making applications more flexible and portable.

2. **Access in Node.js**:

   - In Node.js, environment variables can be accessed using `process.env`.
   - Example:
     ```javascript
     const dbPassword = process.env.DB_PASSWORD;
     console.log(`Database Password: ${dbPassword}`);
     ```

3. **Setting Environment Variables**:

   - Can be set in the terminal before running an application:
     ```bash
     export DB_PASSWORD=mysecretpassword
     node app.js
     ```
   - Or in a `.env` file when using libraries like `dotenv`:
     ```
     DB_PASSWORD=mysecretpassword
     ```
   - Load with:
     ```javascript
     require("dotenv").config();
     ```

4. **Scope**:

   - Environment variables can be set globally or for specific user sessions, and they can also differ between operating systems (e.g., Windows vs. Unix-based systems).

5. **Security**:
   - Storing sensitive information in environment variables is generally more secure than hardcoding them into source code, but care should be taken to avoid leaking these variables (e.g., in version control).

### Example:

Here's a simple example demonstrating how to use an environment variable in a Node.js application:

```javascript
// Accessing an environment variable
const apiKey = process.env.API_KEY;

if (!apiKey) {
  console.error(
    "API key is not defined. Please set the API_KEY environment variable.",
  );
  process.exit(1);
}

// Using the API key in the application
console.log(`Using API Key: ${apiKey}`);
```

This approach makes your application more secure and adaptable to different environments, such as development, testing, and production.
