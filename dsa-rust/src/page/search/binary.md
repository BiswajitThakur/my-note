# Binary Search

**Binary search** is an efficient search algorithm that finds the position of a target value within a **sorted array**. It works by repeatedly dividing the search interval in half. If the target value is less than the middle element, the search continues on the left half; if it is greater, the search proceeds on the right half. This "divide and conquer" approach significantly reduces the number of comparisons needed to find the target.

### How Binary Search Works

1. **Start** with a sorted array and determine the `left` (start) and `right` (end) indices.
2. **Calculate the midpoint**: `mid = (left + right) / 2`.
3. **Compare the middle element** with the target:
   - If the target is equal to the middle element, return the middle index.
   - If the target is less than the middle element, adjust the search to the left half (`right = mid - 1`).
   - If the target is greater than the middle element, adjust the search to the right half (`left = mid + 1`).
4. **Repeat** until the target is found or the search interval is empty (i.e., `left > right`).
5. If the target is not found, return `None`.

### Time Complexity

- **Best Case**: \(O(1)\) - If the middle element is the target.
- **Average Case**: \(O(\log n)\) - Halving the search space with each comparison.
- **Worst Case**: \(O(\log n)\) - When the target is at the extreme ends or not present.

### Advantages of Binary Search

1. **Efficiency**: Significantly faster than linear search for large datasets due to its \(O(\log n)\) time complexity.
2. **Fewer Comparisons**: Reduces the number of comparisons needed by half with each step.
3. **Suitable for Sorted Data**: It leverages the sorted property of data to improve search speed.
4. **Predictable Performance**: Performance remains consistent regardless of data distribution.

### Disadvantages of Binary Search

1. **Requires Sorted Data**: The dataset must be sorted. If the data isn't sorted, you need to sort it first, which can take \(O(n \log n)\) time.
2. **Limited Data Structures**: Works efficiently only on data structures that support random access, like arrays. It's not efficient on linked lists due to the lack of constant-time access.
3. **Complexity in Implementation**: More complex to implement correctly, especially handling edge cases like integer overflow when calculating the midpoint.

### Use Cases

- **Large sorted datasets** where searching speed is crucial.
- **Databases** where the data is indexed and needs to be searched quickly.
- **Binary search trees** and **binary heaps** which are structured for efficient searching.

### Example Code (Rust)

Here's an example of a binary search implementation in Rust:

```rust
{{#include ./../../../code/searching/src/binary.rs}}
```

### Key Differences: Binary Search vs. Linear Search

- **Time Complexity**: Binary search is \(O(\log n)\) vs. Linear search's \(O(n)\).
- **Data Requirement**: Binary search requires sorted data; linear search does not.
- **Performance**: Binary search is faster for large datasets, while linear search is simpler for small or unsorted datasets.

### Summary

Binary search is a powerful, efficient search algorithm ideal for sorted datasets. Its logarithmic complexity makes it much faster than linear search, but it has the constraint of requiring sorted data and works best with data structures that support fast access.
