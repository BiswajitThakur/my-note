# Search Algorithms

Searching is one of the fundamental operations in Data Structures and Algorithms (DSA). It involves locating a specific element in a data structure. The choice of searching technique depends on the structure of the data and the requirements of the application.

---

## **1. Linear Search**

- **Description**: A simple algorithm that checks every element in the list sequentially until the desired element is found or the list ends.
- **Time Complexity**: \(O(n)\)
- **Space Complexity**: \(O(1)\)
- **When to Use**: Works well for small or unsorted datasets.
- **Example**:

```rust
{{ #include ./../../../code/searching/src/linear.rs }}
```

---

## **2. Binary Search**

- **Description**: A fast algorithm that divides the search interval in half repeatedly. It works only on sorted arrays.
- **Time Complexity**: \(O(\log n)\)
- **Space Complexity**: \(O(1)\) (iterative) or \(O(\log n)\) (recursive)
- **When to Use**: Use for large, sorted datasets.
- **Example**:

```rust
{{ #include ./../../../code/searching/src/binary.rs }}
```

---

## **3. Jump Search**

- **Description**: Jumps ahead by a fixed block size and performs a linear search when the jump surpasses the target. Requires a sorted array.
- **Time Complexity**: \(O(\sqrt{n})\)
- **Space Complexity**: \(O(1)\)
- **When to Use**: Useful for uniformly distributed, sorted data.

---

## **4. Interpolation Search**

- **Description**: An improved version of binary search. It estimates the position of the target based on its value and the range of the array. Requires sorted and uniformly distributed data.
- **Time Complexity**: \(O(\log \log n)\) (best case), \(O(n)\) (worst case)
- **Space Complexity**: \(O(1)\)
- **When to Use**: Efficient when data is uniformly distributed.

---

## **5. Exponential Search**

- **Description**: Starts by checking increasingly large ranges and then performs binary search in the appropriate range. Requires a sorted array.
- **Time Complexity**: \(O(\log n)\)
- **Space Complexity**: \(O(1)\)
- **When to Use**: Useful for large datasets with unknown size.

---

| **Algorithm**            | **Best Case** | **Average Case**   | **Worst Case**  | **Space Complexity** |
| ------------------------ | ------------- | ------------------ | --------------- | -------------------- |
| **Linear Search**        | \(O(1)\)      | \(O(n)\)           | \(O(n)\)        | \(O(1)\)             |
| **Binary Search**        | \(O(1)\)      | \(O(\log n)\)      | \(O(\log n)\)   | \(O(1)\)             |
| **Jump Search**          | \(O(1)\)      | \(O(\sqrt{n})\)    | \(O(\sqrt{n})\) | \(O(1)\)             |
| **Interpolation Search** | \(O(1)\)      | \(O(\log \log n)\) | \(O(n)\)        | \(O(1)\)             |
| **Exponential Search**   | \(O(1)\)      | \(O(\log n)\)      | \(O(\log n)\)   | \(O(1)\)             |
