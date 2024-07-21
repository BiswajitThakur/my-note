# What is Java?

Java is a `high-level`, `class-based`, `object-oriented` programming language that is designed to have as few implementation dependencies as possible. It is widely used for building `enterprise-scale applications`, Android apps, and `web applications`. It was developed by Sun Microsystems, which was later acquired by Oracle Corporation.

## High-Level

High-level programming languages are designed to be easy for humans to read and write. They abstract away most of the complex details of the computer's hardware, making it simpler to develop software. High-level languages allow developers to write programs using understandable syntax and constructs, which increases productivity and reduces the likelihood of errors.

### Characteristics of High-Level Languages:

- **Abstraction**: High-level languages provide abstractions that simplify complex tasks, like memory management and hardware interactions.
- **Portability**: Programs written in high-level languages can often be run on different types of computer systems with minimal modification.
- **Ease of Use**: The syntax and semantics are closer to human languages, making them more intuitive and easier to learn.

Examples of high-level languages include Java, Python, C#, and JavaScript.

## Class-Based

Class-based programming languages use classes as the primary means of defining and creating objects. A class is a blueprint for objects that defines attributes (fields) and behaviors (methods) that the objects created from the class will have.

### Characteristics of Class-Based Languages:

- **Encapsulation**: Classes group data and methods that operate on the data together.
- **Inheritance**: Classes can inherit properties and methods from other classes, promoting code reuse and a hierarchical relationship.
- **Polymorphism**: Classes can be used interchangeably if they implement the same interface or inherit from the same base class, enabling flexible and extensible code.

## Object-Oriented

Object-oriented programming (OOP) is a paradigm that uses objects and classes to structure software programs. In OOP, software design is centered around objects, which are instances of classes.

### Core Principles of OOP:

- **Encapsulation**: Bundling data (attributes) and methods (functions) that operate on the data within objects. It restricts direct access to some of an object's components, which is a means of preventing accidental interference and misuse of the data.
- **Inheritance**: Creating new classes from existing ones, inheriting fields and methods. This promotes code reuse and establishes a natural hierarchy.
- **Polymorphism**: Allowing entities to be treated as instances of their parent class rather than their actual class. This enables one interface to be used for a general class of actions.
- **Abstraction**: Hiding complex implementation details and showing only the essential features of the object. It helps in reducing programming complexity and effort.

## Example in Java

```java
// Class definition (class-based)
public class Animal {
    // Fields (attributes)
    private String name;
    private int age;

    // Constructor
    public Animal(String name, int age) {
        this.name = name;
        this.age = age;
    }

    // Methods (behaviors)
    public void speak() {
        System.out.println("Animal speaks");
    }

    // Getters and Setters
    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public int getAge() {
        return age;
    }

    public void setAge(int age) {
        this.age = age;
    }
}

// Inheritance (object-oriented)
public class Dog extends Animal {
    // Constructor
    public Dog(String name, int age) {
        super(name, age);
    }

    // Overriding method (polymorphism)
    @Override
    public void speak() {
        System.out.println("Dog barks");
    }
}

// Main class to test the concepts
public class Main {
    public static void main(String[] args) {
        Animal myAnimal = new Animal("Generic Animal", 5);
        myAnimal.speak(); // Outputs: Animal speaks

        Dog myDog = new Dog("Buddy", 3);
        myDog.speak(); // Outputs: Dog barks
    }
}
```

In this example:

- **High-Level**: The code uses easy-to-read syntax, abstracting away hardware details.
- **Class-Based**: `Animal` and `Dog` are classes defining the blueprint for objects.
- **Object-Oriented**: The code demonstrates encapsulation (fields and methods in classes), inheritance (`Dog` extends `Animal`), and polymorphism (overriding the `speak` method).
