# Loop
Loops in Bash scripting are used to repeatedly execute a block of code as long as a specified condition is true or for a specific number of iterations. Bash provides several types of loops:

1. `for` loop
1. `while` loop
1. `until` loop
1. `select` loop

## `for` Loop
The for loop iterates over a list of items and executes a block of code for each item. It is useful for iterating over arrays, ranges, or command outputs.

### Syntax
```bash
for variable in list; do
    # Commands to execute
done
```
Example
```bash
#!/bin/bash

# Iterate over a list of numbers
for i in 1 2 3 4 5; do
    echo "Iteration $i"
done

# Iterate over a list of filenames
for file in *.sh; do
    echo "Processing $file"
done
```
```
$ ./test.sh
Iteration 1
Iteration 2
Iteration 3
Iteration 4
Iteration 5
Processing test.sh
```

Example
```bash
#!/bin/bash
for i in {1..5}
do
   echo "Welcome $i times"
done
```
```
Welcome 1 times
Welcome 2 times
Welcome 3 times
Welcome 4 times
Welcome 5 times
```

Example
```bash
#!/bin/bash
echo "Bash version ${BASH_VERSION}..."
for i in {0..10..2}
do
  echo "Welcome $i times"
done
```
```
$ ./test.sh
Bash version 5.1.16(1)-release...
Welcome 0 times
Welcome 2 times
Welcome 4 times
Welcome 6 times
Welcome 8 times
Welcome 10 times
```

### Using C-style for Loop
Bash also supports C-style for loops:
```bash
#!/bin/bash

for (( i=0; i<5; i++ )); do
    echo "Iteration $i"
done
```
```
$ ./test.sh
Iteration 0
Iteration 1
Iteration 2
Iteration 3
Iteration 4
```

Example
```bash
for f in $(ls /nas/*.pdf)
do
  printf "File is %s\n" "$f"
done
```

Example - infinite loops
```bash
#!/bin/bash
for (( ; ; ))
do
   echo "infinite loops [ hit CTRL+C to stop]"
done
```

### Conditional exit with break
```bash
for I in 1 2 3 4 5
do
  statements1      #Executed for all values of ''I'', up to a disaster-condition if any.
  statements2
  if (disaster-condition)
  then
    break              #Abandon the loop.
  fi
  statements3              #While good and, no disaster-condition.
done
```

Example
```bash
#!/bin/bash
# Count dns name server in the /etc/resolv.conf if found
for file in /etc/*
do
        # check if file exists in bash using the if #  
    if [ "${file}" == "/etc/resolv.conf" ]
    then
        countNameservers=$(grep -c nameserver /etc/resolv.conf)
        echo "Total dns ${countNameservers} nameservers defined in ${file}"
        break
    fi
done
```
```
$ ./test.sh
Total dns 1 nameservers defined in /etc/resolv.conf
```

### Early continuation with continue statement
```bash
for I in 1 2 3 4 5
do
  statements1      #Executed for all values of ''I'', up to a disaster-condition if any.
  statements2
  if (condition)
  then
    continue   #Go to next iteration of I in the loop and skip statements3
  fi
  statements3
done
```

## `while` Loop
The while loop executes a block of code as long as a specified condition is true. It is useful for situations where the number of iterations is not known in advance.

### Syntax
```bash
while [ condition ]; do
    # Commands to execute
done
```

Example
```bash
#!/bin/bash

count=1

while [ $count -le 5 ]; do
    echo "Count: $count"
    count=$((count + 1))
done
```

## `until` Loop
The until loop is similar to the while loop, but it executes a block of code as long as the specified condition is false. It is essentially the opposite of the while loop.

### Syntax
```bash
until [ condition ]; do
    # Commands to execute
done
```

Example
```bash
#!/bin/bash

count=1

until [ $count -gt 5 ]; do
    echo "Count: $count"
    count=$((count + 1))
done
```

## `select` Loop
The select loop is used to create simple menus. It allows the user to select an option from a list of choices.

### Syntax
```bash
select variable in list; do
    # Commands to execute
done
```
* **variable**: The variable that will hold the value of the selected option.
* **list**: A list of options presented to the user.
* **PS3**: The prompt string that appears before the user input.

Example
```bash
#!/bin/bash

PS3="Please enter your choice: "
options=("Option 1" "Option 2" "Option 3" "Quit")

select opt in "${options[@]}"; do
    case $opt in
        "Option 1")
            echo "You chose option 1"
            ;;
        "Option 2")
            echo "You chose option 2"
            ;;
        "Option 3")
            echo "You chose option 3"
            ;;
        "Quit")
            echo "Exiting..."
            break
            ;;
        *)
            echo "Invalid option $REPLY"
            ;;
    esac
done
```
```
$ ./test.sh
1) Option 1
2) Option 2
3) Option 3
4) Quit
Please enter your choice: 2
You chose option 2
Please enter your choice: 3
You chose option 3
Please enter your choice: 4
Exiting...
```