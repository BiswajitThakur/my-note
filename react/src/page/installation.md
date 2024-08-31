# Installation

.....
.....

## Installing Bun Package Manager

Bun is a fast and modern package manager for JavaScript. Here's how to install it:

1. Open your terminal or command prompt.
2. Run the following command to install Bun:

```bash
npm install -g bun # (make sure to add -g flag)
```

If it's not working then 👇

```bash
curl -fsSL https://bun.sh/install | bash
```

3. Follow the on-screen instructions to complete the installation.

## Creating A React App With Vite

Vite is a build tool that provides a fast and efficient way to set up a React project. Follow these steps to create your first React app:

1. Open your terminal or command prompt.
2. Run the following commands to create a new React project using Vite:

```bash
bun create vite@latest reactthapaapp --template react
cd my-react-app
bun install
```

3. Start the development server:

```bas
bun run dev
```

## Writing Your First "Hello World" Program

Now that your React app is set up, let's write a simple "Hello World" program:

1. Open src/App.jsx in VSCode.
1. Replace the default code with the following:

```jsx
import React from 'react';

function App() {
  return (
Hello World
  );
}
export default App;
```

## Upgrading To React V19 RC

If you're using React v18 and want to upgrade to React v19 Release Candidate, follow these steps:

1. Open your terminal or command prompt.
1. Run the following command to update React:

```bash
bun install react@rc react-dom@rc
```
