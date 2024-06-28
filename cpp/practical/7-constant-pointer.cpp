
#include <iostream>

int main()
{
    /*
    **Constant Pointer (int *const) **
    A constant pointer is a pointer whose address cannot be changed after initialization.
    However, the value at the address it points to can be modified.
    */
    int x0 = 10;
    int y0 = 20;
    int *const ptr0 = &x0; // ptr is a constant pointer to an int
    // ptr0 = &y0;          // Error: ptr cannot point to another address
    *ptr0 = 15;                             // Valid: the value pointed to by ptr can be changed
    std::cout << "x0: " << x0 << std::endl; // Output: x0: 15

    /*
    **Pointer to Constant (const int *) **
    A pointer to a constant is a pointer that can point to different addresses,
    but the value at the address it points to cannot be changed through the pointer.
    */
    int x1 = 10;
    int y1 = 20;
    const int *ptr1 = &x1; // ptr is a pointer to a constant int
    // *ptr1 = 15;         // Error: the value pointed to by ptr cannot be changed
    ptr1 = &y1;                                // Valid: ptr can point to another address
    std::cout << "y1: " << *ptr1 << std::endl; // Output: y1: 20

    /*
    **Constant Pointer to Constant (const int *const) **
    A constant pointer to a constant is a pointer that cannot point to
    another address and the value at the address it points to cannot be changed.
    */
    int x2 = 10;
    const int *const ptr2 = &x2; // ptr is a constant pointer to a constant int
    // ptr2 = &x2;               // Error: ptr cannot point to another address
    // *ptr2 = 15;              // Error: the value pointed to by ptr cannot be changed
    std::cout << "x: " << *ptr2 << std::endl; // Output: x: 10

    return 0;
}
