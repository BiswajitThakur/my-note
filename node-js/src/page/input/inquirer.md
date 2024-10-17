# [Inquirer Package](https://github.com/SBoudrias/Inquirer.js)

The Inquirer package is a popular library in Node.js for building interactive command-line interfaces (CLIs). It allows developers to create prompts that gather input from users in a structured way. Inquirer supports various types of prompts, such as input fields, multiple-choice selections, confirmation prompts, and more.

### Key Features of Inquirer

1. **User-Friendly Prompts**: Inquirer makes it easy to ask questions and gather input through various types of prompts.
2. **Customizable**: You can customize the appearance and behavior of prompts, including validation and default values.
3. **Supports Async/Await**: Inquirer works well with promises and async/await, making it easy to integrate into modern JavaScript applications.
4. **Flexible Input Types**: It supports different types of inputs, such as text input, list selection, checkbox selection, password input, and more.
5. **Validation**: You can add custom validation to ensure that the input meets specific criteria.

### Installation

To use Inquirer in your project, you can install it via npm:

```bash
npm install inquirer
```

### Basic Example

Here’s a simple example of using Inquirer to prompt a user for their name:

```javascript
const inquirer = require("inquirer");

inquirer
  .prompt([
    {
      type: "input",
      name: "name",
      message: "What is your name?",
      validate: (input) => {
        if (input.trim() === "") {
          return "Name cannot be empty!";
        }
        return true;
      },
    },
  ])
  .then((answers) => {
    console.log(`Hello, ${answers.name}!`);
  })
  .catch((error) => {
    console.error("Error occurred:", error);
  });
```

### Common Types of Prompts

1. **Input**: For simple text input.

   ```javascript
   {
       type: 'input',
       name: 'username',
       message: 'Enter your username:',
   }
   ```

2. **List**: For selecting one option from a list.

   ```javascript
   {
       type: 'list',
       name: 'color',
       message: 'Choose your favorite color:',
       choices: ['Red', 'Green', 'Blue'],
   }
   ```

3. **Checkbox**: For selecting multiple options.

   ```javascript
   {
       type: 'checkbox',
       name: 'fruits',
       message: 'Select your favorite fruits:',
       choices: ['Apple', 'Banana', 'Orange', 'Grapes'],
   }
   ```

4. **Confirm**: For yes/no questions.

   ```javascript
   {
       type: 'confirm',
       name: 'proceed',
       message: 'Do you want to continue?',
       default: true,
   }
   ```

5. **Password**: For sensitive input.
   ```javascript
   {
       type: 'password',
       name: 'password',
       message: 'Enter your password:',
       mask: '*',
   }
   ```

### Handling Answers

Inquirer collects the answers in an object, where each key corresponds to the `name` property of each prompt. You can handle the collected data in the `then` method.

### Example with Multiple Prompts

You can chain multiple prompts together to gather a variety of inputs:

```javascript
const inquirer = require("inquirer");

inquirer
  .prompt([
    {
      type: "input",
      name: "name",
      message: "What is your name?",
    },
    {
      type: "list",
      name: "role",
      message: "Select your role:",
      choices: ["Developer", "Designer", "Manager"],
    },
  ])
  .then((answers) => {
    console.log(`Hello, ${answers.name}! You are a ${answers.role}.`);
  })
  .catch((error) => {
    console.error("Error occurred:", error);
  });
```

### Summary

Inquirer is a powerful tool for building interactive command-line applications. It simplifies user input handling and enhances the user experience with a variety of prompt types and customization options. Whether you are building simple scripts or complex command-line tools, Inquirer can help make your application more user-friendly.

---

## Questions & Answers

1. **What is the Inquirer package, and what is its primary use?**

   - **Answer**: The Inquirer package is a popular Node.js library used for creating interactive command-line interfaces (CLIs). It allows developers to prompt users for input in a structured manner through various types of prompts like input fields, lists, checkboxes, and more.

2. **How do you install the Inquirer package in your Node.js project?**

   - **Answer**: You can install the Inquirer package using npm by running the following command in your terminal:
     ```bash
     npm install inquirer
     ```

3. **Can you give an example of a simple text input prompt using Inquirer?**

   - **Answer**: Certainly! Here’s a basic example that prompts the user for their name:

     ```javascript
     const inquirer = require("inquirer");

     inquirer
       .prompt([
         {
           type: "input",
           name: "name",
           message: "What is your name?",
         },
       ])
       .then((answers) => {
         console.log(`Hello, ${answers.name}!`);
       });
     ```

4. **What are some common prompt types available in Inquirer?**

   - **Answer**: Inquirer supports various prompt types, including:
     - `input`: For simple text input.
     - `list`: For selecting one option from a list.
     - `checkbox`: For selecting multiple options.
     - `confirm`: For yes/no questions.
     - `password`: For sensitive input.

5. **How can you validate user input in Inquirer prompts?**

   - **Answer**: You can add a `validate` function to the prompt object, which checks the input and returns a message if it’s invalid. For example:
     ```javascript
     {
         type: 'input',
         name: 'email',
         message: 'Enter your email:',
         validate: (input) => {
             const isValidEmail = /\S+@\S+\.\S+/.test(input);
             return isValidEmail || 'Please enter a valid email address.';
         },
     }
     ```

6. **How can you provide default values for Inquirer prompts?**

   - **Answer**: You can set a `default` property in the prompt object to specify a default value. For example:
     ```javascript
     {
         type: 'input',
         name: 'username',
         message: 'Enter your username:',
         default: 'guest',
     }
     ```

7. **What is the purpose of using the `when` property in an Inquirer prompt?**

   - **Answer**: The `when` property allows you to conditionally show or hide a prompt based on the answers to previous questions. It can be a function that returns a boolean or a boolean value itself. For example:
     ```javascript
     {
         type: 'confirm',
         name: 'wantsMoreInfo',
         message: 'Do you want to provide more information?',
         default: false,
     },
     {
         type: 'input',
         name: 'info',
         message: 'Please provide additional info:',
         when: (answers) => answers.wantsMoreInfo,
     }
     ```

8. **Can you demonstrate how to use Inquirer with async/await syntax?**

   - **Answer**: Yes! Inquirer can be used with async/await by wrapping the prompt call in an async function:

     ```javascript
     const inquirer = require("inquirer");

     const askQuestions = async () => {
       const answers = await inquirer.prompt([
         {
           type: "input",
           name: "name",
           message: "What is your name?",
         },
       ]);
       console.log(`Hello, ${answers.name}!`);
     };

     askQuestions();
     ```

9. **How do you handle multiple prompts in sequence using Inquirer?**

   - **Answer**: You can chain prompts by including them in an array. Each prompt will be presented one after the other:
     ```javascript
     inquirer
       .prompt([
         {
           type: "input",
           name: "name",
           message: "What is your name?",
         },
         {
           type: "list",
           name: "role",
           message: "Select your role:",
           choices: ["Developer", "Designer", "Manager"],
         },
       ])
       .then((answers) => {
         console.log(`Hello, ${answers.name}! You are a ${answers.role}.`);
       });
     ```

10. **What are some common use cases for using Inquirer in Node.js applications?**

    - **Answer**: Common use cases include:
      - Building interactive CLI tools and utilities.
      - Gathering user input for configuration settings.
      - Creating wizards for setup processes.
      - Developing scripts that require user interaction, such as deployment scripts.

11. **What is the purpose of the `filter` property in an Inquirer prompt?**

    - **Answer**: The `filter` property allows you to manipulate the user input before it is returned. It can be a function that receives the user’s input and returns a transformed value. For example, you can use it to trim whitespace or convert input to lowercase:
      ```javascript
      {
          type: 'input',
          name: 'email',
          message: 'Enter your email:',
          filter: (input) => input.trim().toLowerCase(),
      }
      ```

12. **How can you use Inquirer to implement a multi-step wizard?**

    - **Answer**: You can implement a multi-step wizard by chaining multiple prompts and using conditional logic with the `when` property. For example:

      ```javascript
      const inquirer = require("inquirer");

      const step1 = () => {
        return inquirer.prompt([
          {
            type: "input",
            name: "name",
            message: "What is your name?",
          },
        ]);
      };

      const step2 = (answers) => {
        return inquirer.prompt([
          {
            type: "confirm",
            name: "confirm",
            message: `Is your name ${answers.name}?`,
          },
        ]);
      };

      step1()
        .then(step2)
        .then((finalAnswers) => {
          console.log(finalAnswers);
        });
      ```

13. **How can you make a prompt mandatory using Inquirer?**

    - **Answer**: You can make a prompt mandatory by adding validation logic in the `validate` function to ensure that the user provides input. If the input is empty, return an error message. For example:
      ```javascript
      {
          type: 'input',
          name: 'projectName',
          message: 'Enter your project name:',
          validate: (input) => {
              return input.length > 0 || 'Project name cannot be empty!';
          },
      }
      ```

14. **Can you explain how to use Inquirer to prompt for password input?**

    - **Answer**: Inquirer allows you to create a password prompt using the `password` type. You can also mask the input to hide the entered characters. Here’s an example:

      ```javascript
      const inquirer = require("inquirer");

      inquirer
        .prompt([
          {
            type: "password",
            name: "password",
            message: "Enter your password:",
            mask: "*",
          },
        ])
        .then((answers) => {
          console.log("Password entered:", answers.password);
        });
      ```

15. **What is the difference between `prompt` and `promptAsync` methods in Inquirer?**

    - **Answer**: Inquirer primarily uses the `prompt` method for prompting users. The library does not have a `promptAsync` method per se; however, you can use `async/await` with the `prompt` method, effectively achieving asynchronous behavior. There is no distinct `promptAsync` function; it’s about how you choose to handle promises with `prompt`.

16. **How do you set up a default choice for a list prompt?**

    - **Answer**: You can specify a default choice in a list prompt using the `default` property, which should match one of the choices. For example:
      ```javascript
      {
          type: 'list',
          name: 'favoriteFruit',
          message: 'Select your favorite fruit:',
          choices: ['Apple', 'Banana', 'Orange'],
          default: 'Banana',
      }
      ```

17. **How can you implement custom styles or themes for prompts in Inquirer?**

    - **Answer**: Inquirer does not directly support custom themes, but you can customize the prompt display using third-party libraries like `chalk` to style the output. You can also customize the messages and format the text using template literals to add colors and styles.

18. **What are some common mistakes to avoid when using Inquirer?**

    - **Answer**: Some common mistakes include:
      - Forgetting to handle promise rejections with `.catch()`.
      - Not validating user input properly, leading to invalid or empty responses.
      - Using incorrect types or properties in prompts, which may cause errors.
      - Not chaining prompts correctly, leading to unexpected behavior.

19. **Can you explain how Inquirer handles multiple choice questions using checkboxes?**

    - **Answer**: Inquirer allows users to select multiple choices using the `checkbox` type. Users can check or uncheck multiple options. The answers will be returned as an array. Here’s an example:
      ```javascript
      inquirer
        .prompt([
          {
            type: "checkbox",
            name: "hobbies",
            message: "Select your hobbies:",
            choices: ["Reading", "Traveling", "Cooking", "Gaming"],
          },
        ])
        .then((answers) => {
          console.log("Selected hobbies:", answers.hobbies);
        });
      ```

20. **How can you handle dynamic choices in Inquirer prompts?**

    - **Answer**: You can create dynamic choices by using a function for the `choices` property. This function can return an array of choices based on previous answers. Here’s an example:

      ```javascript
      const choices = ["Node.js", "Python", "Java"];

      inquirer
        .prompt([
          {
            type: "list",
            name: "language",
            message: "Select a programming language:",
            choices: choices,
          },
          {
            type: "checkbox",
            name: "projects",
            message: "Select projects to display:",
            choices: () => {
              return choices.map((choice) => ({
                name: `Project for ${choice}`,
                value: choice,
              }));
            },
          },
        ])
        .then((answers) => {
          console.log(answers);
        });
      ```
