# Components

In React.js, a component is one of the core building blocks of the framework. Components allow you to split the UI into independent, reusable pieces, and think about each piece in isolation. Here’s a breakdown of what components are and how they work:

1. ## Types of Components
   - **Function Components**: These are simple JavaScript functions that accept props (short for properties) as an argument and return React elements describing what should appear on the screen.
   - **Class Components**: These are ES6 classes that extend from React.Component and must have a render() method that returns a React element. Class components can also have lifecycle methods.
1. ## Props
   - Props are the inputs to a React component. They are data passed down from a parent component to a child component. A component receives props as a single object and can access individual props using `this.props` in class components or directly via function parameters in function components.
1. ## State
   - State is an object that determines how a component renders and behaves. Unlike props, state is managed within the component (using `this.state` in class components or `useState` hook in function components) and can change over time in response to user actions, network responses, or other factors.
1. ## Lifecycle Methods
   - Lifecycle methods are special methods in class components that allow you to hook into different stages of a component's life (like mounting, updating, and unmounting). Some examples are `componentDidMount`, `componentDidUpdate`, and `componentWillUnmount`.
1. ## Rendering
   - Rendering is the process where the component's output is determined. In React, rendering typically happens when a component’s state or props change. The `render()` method (in class components) or the return statement (in function components) defines what UI will look like based on the current state and props.
1. ## JSX
   - JSX stands for JavaScript XML. It’s a syntax extension that allows you to write HTML elements in JavaScript and place them in the DOM. JSX makes it easier to write and add HTML in React. Components typically return JSX that React converts into real DOM elements.
1. ## Reusability
   - One of the key features of components is reusability. You can create a component once and use it multiple times in different parts of your application. Components can also be composed, meaning one component can be nested inside another.
