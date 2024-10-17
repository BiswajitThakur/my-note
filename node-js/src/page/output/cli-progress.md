# [cli-progress](https://www.npmjs.com/package/cli-progress)

**CLI-Progress** is a Node.js package that provides a customizable progress bar for command-line interface (CLI) applications. It allows developers to visually represent the progress of tasks in the terminal, enhancing the user experience by providing feedback on long-running operations.

### Key Features of CLI-Progress:

1. **Customizable Appearance**: The package offers options to customize the appearance of the progress bar, including colors, width, and format.
2. **Multiple Progress Bars**: You can create multiple progress bars simultaneously, each with its own configuration.
3. **Dynamic Updates**: The progress bar can be updated dynamically based on the progress of ongoing tasks.
4. **Flexible API**: The API is simple and easy to use, making it suitable for both novice and experienced developers.

### Installation:

To install the CLI-Progress package, run the following command:

```bash
npm install cli-progress
```

### Basic Usage:

Here's a simple example demonstrating how to use CLI-Progress to create a progress bar in a Node.js application:

1. **Basic Progress Bar Example**:

```javascript
const { SingleBar, Presets } = require("cli-progress");

// Create a new progress bar instance
const progressBar = new SingleBar(
  {
    format: "{bar} | {percentage}% | {value}/{total} Elapsed: {duration}s",
    hideCursor: true,
  },
  Presets.shades_classic,
);

// Start the progress bar with a total of 100
progressBar.start(100, 0);

// Simulate a task that takes time
let progress = 0;
const interval = setInterval(() => {
  progress += 10;
  progressBar.update(progress); // Update the progress bar

  if (progress >= 100) {
    clearInterval(interval);
    progressBar.stop(); // Stop the progress bar when complete
    console.log("Task complete!");
  }
}, 1000); // Update every second
```

### Explanation of the Example:

- **Importing the Package**: The `SingleBar` class is imported from the `cli-progress` package, along with `Presets` for predefined styles.
- **Creating a Progress Bar**: A new progress bar instance is created with a custom format. The format includes placeholders for the bar, percentage, current value, total value, and elapsed time.
- **Starting the Progress Bar**: The `start` method initializes the progress bar with a total value (100 in this case) and an initial value (0).
- **Updating the Progress Bar**: An interval simulates progress by incrementally updating the progress bar every second. Once the progress reaches 100, the interval is cleared, and the progress bar is stopped.

### Customizing the Progress Bar:

You can customize the appearance and behavior of the progress bar using various options. Here are some examples of customization:

1. **Customizing Bar Length and Colors**:

```javascript
const progressBar = new SingleBar(
  {
    format: "{bar:40} | {percentage}% | {value}/{total}",
    barCompleteChar: "\u2588", // Full block
    barIncompleteChar: "\u2591", // Light shade
    hideCursor: true,
  },
  Presets.shades_classic,
);
```

2. **Using Multiple Progress Bars**:

```javascript
const { MultiBar } = require("cli-progress");

const multiBar = new MultiBar({}, Presets.shades_classic);

// Create two progress bars
const bar1 = multiBar.create(100, 0);
const bar2 = multiBar.create(200, 0);

let progress1 = 0;
let progress2 = 0;

const interval1 = setInterval(() => {
  progress1 += 10;
  bar1.update(progress1);

  if (progress1 >= 100) {
    clearInterval(interval1);
    multiBar.stop();
    console.log("Task 1 complete!");
  }
}, 1000);

const interval2 = setInterval(() => {
  progress2 += 20;
  bar2.update(progress2);

  if (progress2 >= 200) {
    clearInterval(interval2);
    multiBar.stop();
    console.log("Task 2 complete!");
  }
}, 500);
```

### Conclusion:

The CLI-Progress package is a powerful tool for enhancing the interactivity and user experience of command-line applications. By providing a clear visual representation of progress, it helps users understand the state of ongoing tasks, especially in scenarios involving long-running processes.

---

## Questions & Answers

1. **What is the CLI-Progress package used for in Node.js?**

   - **Answer**: CLI-Progress is a Node.js package that provides a customizable progress bar for command-line interface applications, allowing developers to visually represent the progress of tasks.

2. **How do you install the CLI-Progress package?**

   - **Answer**: You can install CLI-Progress using npm by running the command:
     ```bash
     npm install cli-progress
     ```

3. **Can you provide a basic example of using CLI-Progress?**

   - **Answer**: Sure! Here’s a basic example:

     ```javascript
     const { SingleBar } = require("cli-progress");

     const progressBar = new SingleBar();
     progressBar.start(100, 0);

     let progress = 0;
     const interval = setInterval(() => {
       progress += 10;
       progressBar.update(progress);
       if (progress >= 100) {
         clearInterval(interval);
         progressBar.stop();
         console.log("Task complete!");
       }
     }, 1000);
     ```

4. **How can you customize the appearance of the progress bar?**

   - **Answer**: You can customize the appearance of the progress bar by providing options like `format`, `barCompleteChar`, and `barIncompleteChar` when creating the progress bar instance.

5. **What is the difference between `SingleBar` and `MultiBar` in CLI-Progress?**

   - **Answer**: `SingleBar` is used for creating a single progress bar, while `MultiBar` allows for creating and managing multiple progress bars simultaneously, useful for tracking the progress of several tasks at once.

6. **How can you update the progress bar dynamically?**

   - **Answer**: You can update the progress bar dynamically by calling the `update` method with the current progress value. For example:
     ```javascript
     progressBar.update(currentProgress);
     ```

7. **What happens when you call `stop()` on a progress bar?**

   - **Answer**: Calling `stop()` on a progress bar will stop the progress bar and finalize its display, clearing the line in the terminal where the bar was shown.

8. **Can you use colors in the progress bar?**

   - **Answer**: Yes, you can use colors in the progress bar by customizing the `format` and using libraries like Chalk to style the text.

9. **Is CLI-Progress suitable for real-time tasks?**

   - **Answer**: Yes, CLI-Progress is suitable for real-time tasks where progress needs to be updated dynamically, such as file downloads, uploads, or long-running computations.

10. **What are some common use cases for CLI-Progress?**
    - **Answer**: Common use cases for CLI-Progress include file uploads/downloads, data processing tasks, installation scripts, and any long-running tasks where providing feedback to the user is beneficial.

### Additional Questions and Answers on CLI-Progress

11. **How do you create a progress bar with a specific width?**

    - **Answer**: You can set the width of the progress bar by adjusting the `format` option. For example:
      ```javascript
      const progressBar = new SingleBar({
        format: "{bar:30} | {percentage}% | {value}/{total}",
      });
      ```

12. **What is the default behavior of the progress bar when the task is complete?**

    - **Answer**: By default, when the task is complete, the progress bar stops and finalizes its display. You can also customize the completion message in the `stop()` method.

13. **Can you create a progress bar without specifying a total value?**

    - **Answer**: No, a total value is required to initialize the progress bar. It represents the maximum value that the progress bar can reach.

14. **Is it possible to pause and resume the progress bar?**

    - **Answer**: CLI-Progress does not have built-in support for pausing and resuming, but you can achieve this by manually managing the progress state and updating it accordingly.

15. **How do you clear the progress bar from the terminal?**
    - **Answer**: When you call the `stop()` method on a progress bar, it clears the display from the terminal and finalizes the output.
