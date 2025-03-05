mod notation;

struct Stack<T, const SIZE: usize> {
    elements: [T; SIZE],
}
