# Functions
Functions in Bash are reusable blocks of code that can be defined once and called multiple times within a script. They help to organize and modularize your code, making it more readable, maintainable, and reusable.


## Syntax
There are two main ways to define a function in Bash:

### Syntax 1
```bash
function_name() {
    # Commands to execute
}
```

### Syntax 2
```bash
function function_name {
    # Commands to execute
}
```

Example:
```bash
#!/bin/bash

# Define a function
function greet() {
    echo "Hello, $1!"
}

# Call the function
greet "World"
```
```
$ ./test.sh 
Hello, World!
```

### Function with Arguments
Functions can accept arguments, which are accessed inside the function using `$1`, `$2`, etc., where `$1` is the first argument, `$2` is the second, and so on.

Example
```bash
#!/bin/bash

# Define a function with parameters
function sum() {
    local a=$1
    local b=$2
    local result=$((a + b))
    echo "The sum of $a and $b is $result"
}

# Call the function with arguments
sum 5 7
```
```
$ ./test.sh 
The sum of 5 and 7 is 12
```

### Local Variables
Using the `local` keyword, you can define variables inside a function that are local to that function, preventing them from interfering with variables outside the function.

Example
```bash
#!/bin/bash

# Define a function with local variables
function calculate() {
    local x=$1
    local y=$2
    local result=$((x * y))
    echo "The product of $x and $y is $result"
}

# Call the function
calculate 4 5
```
```
$ ./test.sh 
The product of 4 and 5 is 20
```

### Return Values
Functions can return a status code using the `return` keyword. The status code is an integer between `0` and `255`. By convention, `0` indicates success, and non-zero values indicate different types of errors or statuses.

Example
```bash
#!/bin/bash

# Define a function that returns a value
function is_even() {
    local number=$1
    if ((number % 2 == 0)); then
        return 0
    else
        return 1
    fi
}

# Call the function and check the return status
is_even 4
if [ $? -eq 0 ]; then
    echo "The number is even."
else
    echo "The number is odd."
fi
```
```
$ ./test.sh 
The number is even.
```

### Example with Multiple Functions
Here is a script with multiple functions demonstrating how to define, call, and use them:
```bash
#!/bin/bash

# Function to greet a user
greet() {
    echo "Hello, $1!"
}

# Function to add two numbers
add() {
    local a=$1
    local b=$2
    echo "$((a + b))"
}

# Function to check if a number is prime
is_prime() {
    local num=$1
    if ((num <= 1)); then
        return 1
    fi
    for ((i = 2; i <= num / 2; i++)); do
        if ((num % i == 0)); then
            return 1
        fi
    done
    return 0
}

# Call the greet function
greet "Alice"

# Call the add function
result=$(add 3 4)
echo "3 + 4 = $result"

# Call the is_prime function and check the result
number=7
if is_prime $number; then
    echo "$number is a prime number."
else
    echo "$number is not a prime number."
fi
```
```
$ ./test.sh 
Hello, Alice!
3 + 4 = 7
7 is a prime number.
```

### Redirection
```bash
#!/bin/bash

function redirection_in() {
    while read input;
        do
            echo "$input"
        done

} < test.sh

redirection_in

```
In this snippet, we redirect the contents of our test file directly to the standard input of the function.
The read command fetches each line from the standard input.
```
$ ./test.sh 
#!/bin/bash

function redirection_in() {
while read input;
do
echo "$input"
done

} < test.sh

redirection_in
```

### Recursion
We can also use recursion with Bash functions. Let’s explore calculating the n-th Fibonacci number:
```bash
#!/bin/bash

function fibonnaci_recursion() {
    argument=$1
    if [[ "$argument" -eq 0 ]] || [[ "$argument" -eq 1 ]]; then
        echo $argument
    else
        first=$(fibonnaci_recursion $(($argument-1)))
        second=$(fibonnaci_recursion $(($argument-2)))
        echo $(( $first + $second ))
    fi 
}
echo $(fibonnaci_recursion 7)
echo $(fibonnaci_recursion 15)
```
```
$ ./test.sh 
13
610
```

