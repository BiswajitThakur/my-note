# [prompts package](https://www.npmjs.com/package/prompts)

The `prompts` package is a lightweight and flexible library for creating interactive command-line prompts in Node.js. It provides a variety of prompt types and allows for easy customization and validation of user input.

### Key Features of the Prompts Package

1. **Lightweight and Simple**: The `prompts` library is designed to be minimal and easy to use, making it a great choice for developers who need basic prompting functionality without heavy dependencies.

2. **Multiple Prompt Types**: It supports various prompt types, including:

   - Input
   - Select (list)
   - Toggle (yes/no)
   - Multi-select (checkbox)
   - Password

3. **Customizable**: You can customize the prompts easily by providing options for validation, default values, and more.

4. **Asynchronous**: The library uses Promises, allowing you to work with `async/await` syntax for more readable code.

5. **Dynamic Choices**: It allows for dynamic choices in prompts based on previous answers, making it useful for complex interactions.

### Installation

You can install the `prompts` package using npm:

```bash
npm install prompts
```

### Example Usage

Here’s a basic example demonstrating how to use the `prompts` package to create an interactive CLI:

```javascript
const prompts = require("prompts");

const main = async () => {
  const response = await prompts([
    {
      type: "text",
      name: "name",
      message: "What is your name?",
    },
    {
      type: "select",
      name: "color",
      message: "Select your favorite color:",
      choices: [
        { title: "Red", value: "red" },
        { title: "Green", value: "green" },
        { title: "Blue", value: "blue" },
      ],
      initial: 0, // index of the default choice
    },
    {
      type: "confirm",
      name: "confirmation",
      message: `Is ${response.name} your name?`,
      initial: true,
    },
  ]);

  console.log(
    `Hello, ${response.name}! Your favorite color is ${response.color}.`,
  );
};

main();
```

### Additional Features

- **Validation**: You can validate user input to ensure it meets specific criteria.
- **Hidden Input**: For sensitive information like passwords, the prompts package can mask input.
- **Custom Styling**: You can customize the appearance of prompts using the library's options.

### Use Cases

- **Interactive CLI Tools**: Create utilities that require user interaction for configuration or setup.
- **Wizards**: Build multi-step processes that guide users through complex configurations.
- **Scripts**: Enhance scripts that require user input, making them more user-friendly.

### Conclusion

The `prompts` package is a powerful tool for building interactive command-line interfaces in Node.js. Its simplicity, flexibility, and support for various prompt types make it an excellent choice for developers looking to enhance their CLI applications.

---

## Questions and Answers

1. **What is the `prompts` package used for in Node.js?**

   - **Answer**: The `prompts` package is used for creating interactive command-line prompts in Node.js applications. It allows developers to gather input from users through various types of prompts.

2. **How do you install the `prompts` package in a Node.js project?**

   - **Answer**: You can install the `prompts` package by running the following command in your terminal:
     ```bash
     npm install prompts
     ```

3. **Can you provide an example of a basic input prompt using the `prompts` package?**

   - **Answer**: Sure! Here’s a simple example:

     ```javascript
     const prompts = require("prompts");

     (async () => {
       const response = await prompts({
         type: "text",
         name: "username",
         message: "What is your username?",
       });
       console.log(`Hello, ${response.username}!`);
     })();
     ```

4. **What types of prompts does the `prompts` package support?**

   - **Answer**: The `prompts` package supports various types of prompts, including:
     - Text input
     - Select (list)
     - Toggle (yes/no)
     - Multi-select (checkbox)
     - Password input

5. **How can you validate user input with the `prompts` package?**

   - **Answer**: You can validate user input by providing a `validate` function in the prompt options, which checks the input and returns `true` or an error message. For example:
     ```javascript
     const response = await prompts({
       type: "text",
       name: "email",
       message: "Enter your email:",
       validate: (value) => /\S+@\S+\.\S+/.test(value) || "Invalid email!",
     });
     ```

6. **How can you provide default values for prompts in the `prompts` package?**

   - **Answer**: You can set a `initial` property in the prompt options to specify a default value. For example:
     ```javascript
     const response = await prompts({
       type: "select",
       name: "color",
       message: "Select your favorite color:",
       choices: [
         { title: "Red", value: "red" },
         { title: "Green", value: "green" },
       ],
       initial: 1, // default to Green
     });
     ```

7. **Can you demonstrate how to create a multi-select prompt using the `prompts` package?**

   - **Answer**: Certainly! Here’s an example:
     ```javascript
     const response = await prompts({
       type: "multiselect",
       name: "fruits",
       message: "Select your favorite fruits:",
       choices: [
         { title: "Apple", value: "apple" },
         { title: "Banana", value: "banana" },
         { title: "Cherry", value: "cherry" },
       ],
     });
     console.log("Selected fruits:", response.fruits);
     ```

8. **How do you handle sensitive input, like passwords, with the `prompts` package?**

   - **Answer**: You can handle sensitive input by using the `password` prompt type, which masks the input. Here’s an example:
     ```javascript
     const response = await prompts({
       type: "password",
       name: "password",
       message: "Enter your password:",
       mask: "*",
     });
     ```

9. **How can you create dynamic choices based on previous answers in the `prompts` package?**

   - **Answer**: You can create dynamic choices by using a function for the `choices` property that returns an array based on previous answers. For example:
     ```javascript
     const response = await prompts([
       {
         type: "select",
         name: "color",
         message: "Select a color:",
         choices: [
           { title: "Red", value: "red" },
           { title: "Green", value: "green" },
         ],
       },
       {
         type: "select",
         name: "shade",
         message: "Select a shade:",
         choices: (prev) =>
           prev.color === "red"
             ? [
                 { title: "Light Red", value: "lightRed" },
                 { title: "Dark Red", value: "darkRed" },
               ]
             : [
                 { title: "Light Green", value: "lightGreen" },
                 { title: "Dark Green", value: "darkGreen" },
               ],
       },
     ]);
     ```

10. **What are some best practices for using the `prompts` package in your applications?**
    - **Answer**: Best practices include:
      - Always validate user input to avoid errors.
      - Provide clear and concise messages for prompts.
      - Use default values where appropriate to guide users.
      - Consider the flow of prompts to ensure a smooth user experience.
