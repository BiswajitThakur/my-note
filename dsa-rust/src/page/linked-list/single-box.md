# Single Linked List (Box)

#### **What is a Singly Linked List?**

- A **Singly Linked List** is a linear data structure where each element (node) points to the next node in the sequence.
- It consists of a sequence of nodes, where each node contains:
  1. **Data**: The actual value stored.
  2. **Pointer**: A reference (or link) to the next node.

---

#### **Structure of a Node**

```
+-----------+------------+
|   Data    |   Pointer  |
+-----------+------------+
```

Example of a Singly Linked List:

```
+---+    +---+    +---+    +---+
| 1 | -> | 2 | -> | 3 | -> | X |
+---+    +---+    +---+    +---+
```

---

#### **Key Properties**

1. **Dynamic Size**: The list can grow or shrink dynamically.
2. **Sequential Access**: Nodes must be accessed sequentially from the head.
3. **Unidirectional**: Traversal is only possible in one direction (from head to tail).
4. **No Backtracking**: Once you move forward, you cannot go back.

---

#### **Basic Operations**

1. **Insert at Head**

   - Add a new node at the beginning.
   - Example:
     ```
     Before: HEAD -> 2 -> 3 -> X
     After:  HEAD -> 1 -> 2 -> 3 -> X
     ```

2. **Insert at Tail**

   - Add a new node at the end.
   - Example:
     ```
     Before: HEAD -> 1 -> 2 -> X
     After:  HEAD -> 1 -> 2 -> 3 -> X
     ```

3. **Remove a Node**

   - Delete a node with a specific value.
   - Example:
     ```
     Before: HEAD -> 1 -> 2 -> 3 -> X
     Remove: 2
     After:  HEAD -> 1 -> 3 -> X
     ```

4. **Search**
   - Find if a value exists in the list.
   - Example:
     ```
     List: HEAD -> 1 -> 2 -> 3 -> X
     Search for: 2
     Output: Found
     ```

---

#### **Advantages**

1. **Efficient Insertion/Deletion**: O(1) at the head.
2. **Dynamic Size**: No need to predefine the size.

---

#### **Disadvantages**

1. **Sequential Access Only**: O(n) to access an element.
2. **More Memory Usage**: Each node requires extra memory for the pointer.

---

#### **Example Implementation in Rust**

```rust
{{#include ./../../../code/linked-list/src/box_single_linked_list.rs}}
```

---

#### **ASCII Visualization of a Singly Linked List**

Insertion at Head:

```
Before:   HEAD -> 2 -> 3 -> X
Insert:   1
After:    HEAD -> 1 -> 2 -> 3 -> X
```

Traversal:

```
HEAD -> 1 -> 2 -> 3 -> X
   |      |      |
  Data   Data   Data
```
