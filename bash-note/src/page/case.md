# Case
The case statement in Bash is a powerful control structure used to execute different blocks of code based on the value of a variable or an expression. It is similar to the switch statement found in other programming languages. The case statement is especially useful when you need to compare a single variable against multiple possible values and perform different actions based on the match.

## Syntax
The basic syntax of the case statement is as follows:

```bash
case expression in
    pattern1)
        # Commands to execute if expression matches pattern1
        ;;
    pattern2)
        # Commands to execute if expression matches pattern2
        ;;
    ...
    *)
        # Commands to execute if no pattern matches (default case)
        ;;
esac
```

* **expression**: The variable or value being compared.
* **pattern**: The possible values to match against the expression. Patterns can include wildcard characters (*, ?, []).
* **;;**: Terminates each block of commands.

Example 0:
```bash
#!/bin/bash

fruit="apple"

case $fruit in
    "apple")
        echo "Apple is red"
        ;;
    "banana")
        echo "Banana is yellow"
        ;;
    "orange")
        echo "Orange is orange"
        ;;
    *)
        echo "Unknown fruit"
        ;;
esac
```
```
$ ./test.sh
Apple is red
```

Example 1:
```bash
#!/bin/bash

read -p "Enter a number between 1 and 3: " number

case $number in
    1)
        echo "You entered one."
        ;;
    2)
        echo "You entered two."
        ;;
    3)
        echo "You entered three."
        ;;
    *)
        echo "Invalid number. Please enter a number between 1 and 3."
        ;;
esac
```
```
$ ./test.sh
Enter a number between 1 and 3: 2
You entered two.
$ ./test.sh
Enter a number between 1 and 3: 99
Invalid number. Please enter a number between 1 and 3.
$ ./test.sh
Enter a number between 1 and 3: 
Invalid number. Please enter a number between 1 and 3.
```

Example 2:
```bash
#!/bin/bash

read -p "Enter a file extension: " extension

case $extension in
    txt)
        echo "Text file"
        ;;
    jpg | jpeg)
        echo "JPEG image file"
        ;;
    png)
        echo "PNG image file"
        ;;
    *)
        echo "Unknown file type"
        ;;
esac
```
```
$ ./test.sh
Enter a file extension: png
PNG image file
$ ./test.sh
Enter a file extension: jpg
JPEG image file
$ ./test.sh
Enter a file extension: jpeg
JPEG image file
$ ./test.sh
Enter a file extension: apk
Unknown file type
```


Example 3:
```bash
#!/bin/bash

read -p "Enter a character: " char

case $char in
    [a-z])
        echo "You entered a lowercase letter."
        ;;
    [A-Z])
        echo "You entered an uppercase letter."
        ;;
    [0-9])
        echo "You entered a digit."
        ;;
    ?)
        echo "You entered a special character."
        ;;
    *)
        echo "Invalid input."
        ;;
esac
```
```
$ ./test.sh
Enter a character: e
You entered a lowercase letter.
$ ./test.sh
Enter a character: E
You entered an uppercase letter.
$ ./test.sh
Enter a character: 8
You entered a digit.
$ ./test.sh
Enter a character: @
You entered a special character.
$ ./test.sh
Enter a character: abc
Invalid input.
```

Example 4:
```bash
#!/bin/bash

while true; do
    echo "Menu:"
    echo "1. Print date"
    echo "2. List files"
    echo "3. Exit"
    read -p "Enter your choice: " choice

    case $choice in
        1)
            date
            ;;
        2)
            ls -l
            ;;
        3)
            echo "Goodbye!"
            break
            ;;
        *)
            echo "Invalid choice, please try again."
            ;;
    esac
done

```

```
$ ./test.sh
Menu:
1. Print date
2. List files
3. Exit
Enter your choice: 1
Thu Jun 27 10:01:39 AM IST 2024
Menu:
1. Print date
2. List files
3. Exit
Enter your choice: 2
total 8
drwxrwxr-x 1 eagle eagle 470 Jun 27 10:01 book
-rw-rw-r-- 1 eagle eagle 107 Jun 27 06:30 book.toml
drwxrwxr-x 1 eagle eagle  28 Jun 27 06:41 src
-rwxr--r-- 1 eagle eagle 429 Jun 27 10:01 test.sh
Menu:
1. Print date
2. List files
3. Exit
Enter your choice: 10
Invalid choice, please try again.
Menu:
1. Print date
2. List files
3. Exit
Enter your choice: 3
Goodbye!
```