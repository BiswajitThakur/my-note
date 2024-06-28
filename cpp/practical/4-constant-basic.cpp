#include <iostream>

int main()
{
    /*
    The const keyword is used to declare constant variables.
    Once a constant is initialized, its value cannot be changed.
    */
    const int MX = 100;
    // MX = 99;  // Error
    std::cout << MX << '\n';
    return 0;
}