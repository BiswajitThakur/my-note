#include <iostream>
using namespace std;

int main()
{
    int a = 5;  // initial value: 5 (c-like initialization )
    int b(3);   // initial value: 3 (constructor initialization)
    int c{2};   // initial value: 2 (uniform initialization)
    int result; // initial value undetermined

    a = a + b;
    result = a - c;
    cout << result << '\n';

    return 0;
}
