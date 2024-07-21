# Why Java is Platform Independent?

Java is platform independent programming language as Java doesn’t require the entire code to be rewritten for all the different platforms. It supports platform independence using Java bytecode and Java Virtual Machine. Java compiler Javac converts the program code into byte code. This byte code is platform-independent and can run on any JVM operating system. JVM interprets the byte code to machine code, and the program is executed.

## Step-by-Step Execution of Java Program

- Java compiler does not produce native executable files or code for a specific platform.
- Instead, it generates a special format known as byte code during the compilation process.
- Byte code serves as a set of machine instructions for a Java processor chip called Java Virtual Machine (JVM).
- Java Virtual Machine (JVM) interprets and executes the byte code.
- The byte code is platform-independent, meaning it remains the same across different operating systems and hardware configurations.
- JVM acts as a special Java interpreter, taking byte code as input and providing the output of the program after interpretation and execution.

**Note**: A compiler translates the entire program at once into machine-readable code. An Interpreter also converts the program to machine code, but it does so by translating it instruction-by-instruction or line-by-line.

The interpreter then converts the byte code into the native code. Native code is just like machine language code, which can be compiled to run with a particular processor and its set of instructions. Thus, native code can be executed by an individual operating system. Java Byte Code is platform-independent but natively executable code generated from byte code using an interpreter is highly platform-dependent and cannot run on other platforms.

## Why Java is Platform Independent But JVM is Not?

Java is not completely platform-independent. The Javac compiler first compiles the High-Level program code written by the programmer, and byte code is formed. This byte code is platform-independent but requires a Just In Time (JIT) interpreter/compiler. JVM, which has JIT, interprets/compiles the byte code, converts it into machine code, and executes the program.

This Interpreter, Java Virtual Machine, is platform-dependent. Different systems have different JVMs. For example, MAC OS has a different JVM, and Windows will have a different JVM. This JVM is capable of reading the .class file or byte code.

So, we can conclude that Java is independent of the platform using Java Byte Code. But the execution of Byte Code on any platform relies on Java Virtual Machine, which is platform-specific.

## Important Points to Remember

- The primary objective in the creation of the Java Programming Language was to establish it as a portable, straightforward, and secure programming language.
- Apart from these, it also supports some excellent features that contribute to the popularity of Java, like Object-Oriented programming, Platform independence, Portability, Dynamic in nature, Architecture neutral, Multithreaded, Robust, Interpreted, etc.
- Java provides Platform Independence by making use of Java Byte Code. Java Byte Code or .class file is generated during the compilation of the code. This Byte Code is platform-independent and can run on any system regardless of the platform it is built upon.
- Byte Code differs from machine code which cannot be transferred to another platform. And Java Virtual Machine, an Interpreter, can run the Byte Code.
- JVM recognizes the platform and converts the Byte Code into machine code which can finally be read and executed.
