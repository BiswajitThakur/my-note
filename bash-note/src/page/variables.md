# Variables & Data Types
In Bash scripting, variables are used to store data that can be referenced and manipulated throughout the script. Variables in Bash do not require explicit declaration of their data types. Here are the basics of using variables in Bash:

## 1. Declaring and Assigning Variables
* Variables are declared and assigned without a = sign and no spaces around it.
* Variable names can include letters, numbers, and underscores, but they cannot start with a number.
```bash
#!/bin/bash

name="John Doe"
age=25
echo "Name: $name"
echo "Age: $age"
```
```
Name: John Doe
Age: 25
```

## 2. Accessing Variables
* To access the value of a variable, prefix its name with a dollar sign **$**.
```bash
echo "Name: $name"
echo "Age: $age"
```

## 3. Variable Types
While Bash variables are typically strings, they can be used in different contexts to represent various data types:

* **String**: By default, variables are treated as strings.
* **Integer**: Variables can be used as integers in arithmetic operations.
* **Array**: Bash supports one-dimensional arrays.

### Strings
String manipulation is straightforward in Bash:

```bash
# String assignment
greeting="Hello, World!"

# String concatenation
full_greeting="${greeting} How are you?"

echo $full_greeting
```
```
Hello, World! How are you?
```

### Integers
To perform arithmetic operations, use the let, expr, (( )), or [] syntax:
```bash
# Integer assignment
num1=10
num2=20

# Using let
let sum=num1+num2
echo "Sum: $sum"

# Using expr
sum=$(expr $num1 + $num2)
echo "Sum: $sum"

# Using double parentheses
sum=$((num1 + num2))
echo "Sum: $sum"

# Using single brackets
if [ $num1 -lt $num2 ]; then
    echo "$num1 is less than $num2"
fi
```
```
Sum: 30
Sum: 30
Sum: 30
10 is less than 20
```

### Arrays
Bash supports indexed arrays but not associative arrays by default:
```bash
# Array assignment
fruits=("Apple" "Banana" "Cherry")

# Accessing array elements
echo "First fruit: ${fruits[0]}"
echo "All fruits: ${fruits[@]}"

# Adding elements
fruits[3]="Date"

# Looping through array
for fruit in "${fruits[@]}"; do
    echo "$fruit"
done
```
```
First fruit: Apple
All fruits: Apple Banana Cherry
Apple
Banana
Cherry
Date
```

### Special Variables
Bash provides several special variables:

* **$0**: The name of the script.
* **$1**, **$2**, ...: The first, second, ... arguments to the script.
* **$#**: The number of arguments.
* **$@**: All arguments.
* **$?**: The exit status of the last command.
* **$$**: The process ID of the script.
* **$USER**: The current user's username.
* **$HOSTNAME**: The hostname of the machine.
* **$SECONDS**: The number of seconds since the script started.
```bash
#!/bin/bash

echo "Script name: $0"
echo "First argument: $1"
echo "Number of arguments: $#"
echo "All arguments: $@"
sleep 1
echo "Last command exit status: $?"
echo "Script PID: $$"
sleep 2
echo "Current user: $USER"
echo "Hostname: $HOSTNAME"
echo "Seconds since script started: $SECONDS"
```
```
$ ./test.sh arg1 arg2 arg3
Script name: ./test.sh
First argument: arg1
Number of arguments: 3
All arguments: arg1 arg2 arg3
Last command exit status: 0
Script PID: 7812
Current user: eagle
Hostname: eagle-quantum
Seconds since script started: 3
```

### Environment Variables
Environment variables are system-wide variables available to all processes:

```bash
# Displaying environment variables
echo "Home directory: $HOME"
echo "Current directory: $PWD"
echo "Path: $PATH"

# Setting environment variables
export MY_VAR="Hello"
echo "My variable: $MY_VAR"
```
```
Home directory: /home/eagle
Current directory: /home/eagle/tutorial/bash-note
Path: /home/eagle/.bun/bin:/home/eagle/.bun/bin:/home/eagle/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin:/snap/bin:/usr/local/go/bin:/home/eagle/go/bin:/home/eagle/node-v20.14.0-linux-x64/bin:/home/eagle/node-v20.14.0-linux-x64/bin
My variable: Hello
```
