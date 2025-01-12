# Sorting Algorithms

Sorting is the process of arranging elements in a particular order (e.g., ascending or descending). Sorting is a fundamental operation in data structures and algorithms because many algorithms (like binary search) require sorted data. Sorting algorithms are categorized based on their efficiency, stability, and whether they work in-place.

---

## **1. Bubble Sort**

- **Description**: Repeatedly swaps adjacent elements if they are in the wrong order. It "bubbles" the largest elements to the end.
- **Time Complexity**:
  - Best Case: O(n) (when the array is already sorted)
  - Average Case: O(n^2)
  - Worst Case: O(n^2)
- **Space Complexity**: O(1)
- **Stable**: Yes
- **In-Place**: Yes
- **Example**:

```rust
{{ #include ./../../../code/sorting/src/bubble.rs }}
```

---

## **2. Selection Sort**

- **Description**: Repeatedly finds the smallest element from the unsorted part and swaps it with the first unsorted element.
- **Time Complexity**:
  - Best Case: O(n^2)
  - Average Case: O(n^2)
  - Worst Case: O(n^2)
- **Space Complexity**: O(1)
- **Stable**: No
- **In-Place**: Yes
- **Example**:

```rust
{{ #include ./../../../code/sorting/src/selection.rs }}
```

---

## **3. Insertion Sort**

- **Description**: Builds the sorted array one element at a time by inserting each element into its correct position.
- **Time Complexity**:
  - Best Case: \(O(n)\) (when the array is already sorted)
  - Average Case: \(O(n^2)\)
  - Worst Case: \(O(n^2)\)
- **Space Complexity**: \(O(1)\)
- **Stable**: Yes
- **In-Place**: Yes

---

## **4. Merge Sort**

- **Description**: Divides the array into halves, recursively sorts them, and then merges the sorted halves.
- **Time Complexity**:
  - Best Case: \(O(n \log n)\)
  - Average Case: \(O(n \log n)\)
  - Worst Case: \(O(n \log n)\)
- **Space Complexity**: \(O(n)\)
- **Stable**: Yes
- **In-Place**: No

---

## **5. Quick Sort**

- **Description**: Divides the array into two parts using a pivot and recursively sorts the partitions.
- **Time Complexity**:
  - Best Case: \(O(n \log n)\)
  - Average Case: \(O(n \log n)\)
  - Worst Case: \(O(n^2)\) (when the pivot is poorly chosen)
- **Space Complexity**: \(O(\log n)\) (for recursive stack)
- **Stable**: No
- **In-Place**: Yes

---

## **6. Heap Sort**

- **Description**: Converts the array into a binary heap, then repeatedly extracts the maximum/minimum element.
- **Time Complexity**:
  - Best Case: \(O(n \log n)\)
  - Average Case: \(O(n \log n)\)
  - Worst Case: \(O(n \log n)\)
- **Space Complexity**: \(O(1)\)
- **Stable**: No
- **In-Place**: Yes

---

## **Comparison of Sorting Algorithms**

| **Algorithm**  | **Best Case**   | **Average Case** | **Worst Case**  | **Space Complexity** | **Stable** | **In-Place** |
| -------------- | --------------- | ---------------- | --------------- | -------------------- | ---------- | ------------ |
| Bubble Sort    | \(O(n)\)        | \(O(n^2)\)       | \(O(n^2)\)      | \(O(1)\)             | Yes        | Yes          |
| Selection Sort | \(O(n^2)\)      | \(O(n^2)\)       | \(O(n^2)\)      | \(O(1)\)             | No         | Yes          |
| Insertion Sort | \(O(n)\)        | \(O(n^2)\)       | \(O(n^2)\)      | \(O(1)\)             | Yes        | Yes          |
| Merge Sort     | \(O(n \log n)\) | \(O(n \log n)\)  | \(O(n \log n)\) | \(O(n)\)             | Yes        | No           |
| Quick Sort     | \(O(n \log n)\) | \(O(n \log n)\)  | \(O(n^2)\)      | \(O(\log n)\)        | No         | Yes          |
| Heap Sort      | \(O(n \log n)\) | \(O(n \log n)\)  | \(O(n \log n)\) | \(O(1)\)             | No         | Yes          |
