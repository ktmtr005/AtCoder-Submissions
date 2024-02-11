#include <iostream>
using namespace std;
int main() {
    int a, b;
    cin >> a >> b;
    if ((b - 1) % 3 - (a - 1) % 3 == 1 && b - a == 1)
        cout << "Yes" << endl;
    else
        cout << "No" << endl;
}