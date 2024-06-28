/*
**const vs constexpr**
const:
    Can be used for both compile-time and runtime constants.
    he value must be initialized but can be computed at runtime.
constexpr:
    Enforces that the value must be computable at compile time.
    It is used for constants that need to be evaluated during the
    compilation phase.
*/

#include <iostream>

const int runtimeConst = []()
{ return 10; }();                    // Computed at runtime
constexpr int compileTimeConst = 20; // Computed at compile time

constexpr int add(int a, int b)
{
    return a + b;
}

int main()
{
    std::cout << "Runtime constant: " << runtimeConst << std::endl;
    std::cout << "Compile-time constant: " << compileTimeConst << std::endl;

    constexpr int sum = add(5, 7); // sum is computed at compile time
    std::cout << "Sum: " << sum << std::endl;

    return 0;
}
