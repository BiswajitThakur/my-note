// auto and decltype

#include <iostream>

auto add(int a, int b) -> int
{
    return a + b;
}

int main()
{
    /*
    The auto keyword is used for automatic type deduction. It lets the compiler
    deduce the type of a variable from its initializer. This can make code more
    readable and reduce the need for repetitive type declarations.
    */
    int foo = 99;
    auto bar = 1; // the same as: int bar = 1;
    auto sum = add(foo, bar);
    std::cout << sum << '\n';

    /*
    The decltype keyword is used to query the type of an expression. It returns
    the type of the expression without evaluating it. This is particularly useful
    for templates and for writing generic code.
    */
    float f = 0.1;
    decltype(f) bb = 0.2; // the same as: float bb;
    decltype(bb) s = f + bb;
    std::cout << s << '\n';

    int a = 3;
    double b = 4.5;
    decltype(a + b) c = a + b; // c is deduced to be of type double
    std::cout << c << '\n';

    return 0;
}
