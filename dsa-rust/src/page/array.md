# Array

#### Definition:

An **array** in Rust is a fixed-size, contiguous collection of elements of the same type. Arrays are stored on the stack and are useful when you know the number of elements in advance.

#### Syntax:

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];  // An array of 5 integers
```

- `i32`: Type of elements stored in the array.
- `5`: Size of the array (number of elements).

You can also initialize arrays with the same value for all elements:

```rust
let arr = [0; 5];  // An array of 5 elements, all initialized to 0
```

#### Accessing Elements:

You can access array elements using index notation:

```rust
let first_element = arr[0];  // Access the first element
let second_element = arr[1];  // Access the second element
```

Rust performs **bounds checking** at runtime, meaning accessing an index outside the bounds of the array will cause a **panic**.

#### Slices:

A **slice** is a reference to a portion of an array. Slices allow you to work with parts of arrays without copying data.

```rust
let slice = &arr[1..4];  // A slice containing elements from index 1 to 3
```

#### Arrays vs Vectors:

- Arrays have a **fixed size**, whereas vectors (`Vec<T>`) are **dynamically sized**.
- Arrays are more efficient when the size is known at compile time, but vectors offer more flexibility with resizing.

#### Example:

```rust
fn main() {
    let arr = [10, 20, 30, 40, 50];

    for i in 0..arr.len() {
        println!("Element at index {}: {}", i, arr[i]);
    }

    let slice = &arr[1..4];  // Taking a slice of the array
    println!("Slice: {:?}", slice);
}
```

### Questions:

**1. What is an array in Rust, and how does it differ from a vector?**

- **Answer**:
  An array in Rust is a fixed-size, stack-allocated collection of elements of the same type. Its size is known at compile time. A vector, on the other hand, is a dynamically sized, heap-allocated collection, which can grow or shrink at runtime.

  Example:

  ```rust
  let arr: [i32; 3] = [1, 2, 3];  // Array
  let vec = vec![1, 2, 3];        // Vector
  ```

**2. What is bounds checking in Rust arrays?**

- **Answer**:
  Rust ensures that when accessing an array, the index is within the valid range of the array. If an index is out of bounds, Rust will cause a runtime panic, preventing access to invalid memory.

  Example:

  ```rust
  let arr = [1, 2, 3];
  println!("{}", arr[5]);  // This will cause a panic due to out-of-bounds access.
  ```

**3. How do you create a slice from an array in Rust?**

- **Answer**:
  A slice is a reference to a portion of an array. You can create a slice by specifying a range of indices.

  Example:

  ```rust
  let arr = [1, 2, 3, 4, 5];
  let slice = &arr[1..4];  // A slice of elements at index 1, 2, 3
  println!("{:?}", slice);  // Output: [2, 3, 4]
  ```

**4. How can you iterate over the elements of an array in Rust?**

- **Answer**:
  You can iterate over an array using a `for` loop or the `iter()` method.

  Example:

  ```rust
  let arr = [10, 20, 30, 40];

  // Using a for loop
  for element in arr.iter() {
      println!("{}", element);
  }

  // Using a traditional index-based loop
  for i in 0..arr.len() {
      println!("{}", arr[i]);
  }
  ```

**5. Can arrays in Rust store elements of different types?**

- **Answer**:
  No, arrays in Rust can only store elements of the same type. If you need to store multiple types, you can use an `enum` or a tuple.

  Example using `enum`:

  ```rust
  enum Value {
      Int(i32),
      Float(f64),
  }

  let arr: [Value; 3] = [Value::Int(10), Value::Float(3.14), Value::Int(42)];
  ```

### Practice Problem:

**Problem:**  
Write a function that takes an array of integers and returns the sum of all elements in the array.

**Solution:**

```rust
fn sum_of_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in arr {
        sum += num;
    }
    sum
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let sum = sum_of_array(&arr);
    println!("Sum of array: {}", sum);  // Output: 15
}
```
