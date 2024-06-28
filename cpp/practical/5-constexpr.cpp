/*
The constexpr keyword is used to define constants that are
evaluated at compile time. constexpr variables must be
initialized with constant expressions and can be used in
contexts that require compile-time constants, such as array sizes.
*/

#include <iostream>

constexpr int MAX_SIZE = 100; // MAX_SIZE is a compile-time constant

constexpr int square(int x)
{
    return x * x;
}

int main()
{
    std::cout << MAX_SIZE << '\n';
    std::cout << square(7) << '\n';

    return 0;
}
