# Else If Statement
The if statement allows you to execute a block of code only if a certain condition is true. The else and elif (else if) statements provide additional conditions and alternative code blocks.

## Operators
| Comparisons | Function |
|:------------|:--------:|
| **Integer** | |
| -gt | Greater-than |
| -lt | Less-than |
| -ge | Greater-than-or-equal-to |
| -le | Less-than-or-equal-to |
| -eq | Equal |
| -ne | Not-equal |
| **String** | |
| -z | Tests for empty string |
| = | Equal strings |
| != | Not-equal strings |
| **Logical Operations** | |
| -a | Logical AND |
| -o | Logical OR |
| ! | Logical NOT |
| **File Tests** | |
| -f FILE | True if the FILE exists and is a regular file (not a directory or device). |
| -s FILE | File is not empty |
| -r FILE | True if the FILE exists and is readable. |
| -w FILE | True if the FILE exists and is writable. |
| -x FILE | True if the FILE exists and is executable. |
| -d FILE | True if the FILE exists and is a directory. |
| -h FILE | True if the FILE exists and is a symbolic link. |

```bash
#!/bin/bash

# Basic if statement
if [ condition ]; then
    # Commands to execute if condition is true
fi

# if-else statement
if [ condition ]; then
    # Commands to execute if condition is true
else
    # Commands to execute if condition is false
fi

# if-elif-else statement
if [ condition1 ]; then
    # Commands to execute if condition1 is true
elif [ condition2 ]; then
    # Commands to execute if condition2 is true
else
    # Commands to execute if all conditions are false
fi
```

Example 0:
```bash
#!/bin/bash

number=10

if [ $number -gt 15 ]; then
    echo "The number is greater than 15"
elif [ $number -eq 15 ]; then
    echo "The number is equal to 15"
else
    echo "The number is less than 15"
fi
```
```
The number is less than 15
```

Example 1:
```bash
#!/bin/bash

number=10

if [ $number -gt 5 ] && [ $number -lt 15 ]; then
    echo "The number is between 5 and 15"
else
    echo "The number is not between 5 and 15"
fi
```
```
The number is between 5 and 15
```

Example 2:
```bash
#!/bin/bash

var1=10
var2=20

if [ $var1 -gt 5 -a $var2 -lt 30 ]; then
    echo "Both conditions are true"
else
    echo "One or both conditions are false"
fi
```

Example 3:
```bash
#!/bin/bash

number=3

if [ $number -lt 5 ] || [ $number -gt 15 ]; then
    echo "The number is less than 5 or greater than 15"
else
    echo "The number is between 5 and 15"
fi
```
```
The number is less than 5 or greater than 15
```

Example 4:
```bash
#!/bin/bash

var1=10
var2=20

if [ $var1 -gt 15 -o $var2 -lt 30 ]; then
    echo "At least one condition is true"
else
    echo "Both conditions are false"
fi
```

Example 5:
```bash
#!/bin/bash

number=10

if ! [ $number -eq 5 ]; then
    echo "The number is not equal to 5"
else
    echo "The number is equal to 5"
fi
```

Example 6:
```bash
#!/bin/bash

number=20

if ([ $number -gt 5 ] && [ $number -lt 15 ]) || [ $number -eq 20 ]; then
    echo "The number is either between 5 and 15, or it is 20"
else
    echo "The number is not in the specified range"
fi
```
```
$ ./test.sh
The number is either between 5 and 15, or it is 20
```

Example 7:
```bash
#!/bin/bash

var1=10
var2=20

# Using && for logical AND
if [[ $var1 -gt 5 && $var2 -lt 30 ]]; then
    echo "Both conditions are true"
else
    echo "One or both conditions are false"
fi

# Using || for logical OR
if [[ $var1 -gt 15 || $var2 -lt 30 ]]; then
    echo "At least one condition is true"
else
    echo "Both conditions are false"
fi
```

Example 8:
```bash
#!/bin/bash

var1=10
var2=20
var3=30

if [[ ($var1 -gt 5 && $var2 -lt 25) || $var3 -eq 30 ]]; then
    echo "Complex condition is true"
else
    echo "Complex condition is false"
fi
```
```
$ ./test.sh
Complex condition is true
```

Example 9:
```bash
#!/bin/bash

var1=10
var2=20

# Using logical AND with &&
if [[ $var1 -gt 5 && $var2 -lt 30 ]]; then
    echo "Both conditions are true"
else
    echo "One or both conditions are false"
fi

# Using logical OR with ||
if [[ $var1 -gt 15 || $var2 -lt 30 ]]; then
    echo "At least one condition is true"
else
    echo "Both conditions are false"
fi
```
```
$ ./test.sh
Both conditions are true
At least one condition is true
```

Example 10:
```bash
#!/bin/bash

file="book.toml"

if [ -f "$file" ] && [ -w "$file" ]; then
    echo "The file exists and is writable"
else
    echo "The file either does not exist or is not writable"
fi
```
```
$ ls
book  book.toml  src  test.sh
$ ./test.sh
The file exists and is writable
```


Example 11:
```bash
#!/bin/bash

dir="my_directory"
file="my_file.txt"

if [ -d "$dir" ] || [ -f "$file" ]; then
    echo "Either the directory exists or the file exists"
else
    echo "Neither the directory nor the file exists"
fi
```
```
$ ls
book  book.toml  src  test.sh
$ ./test.sh
Neither the directory nor the file exists
```
