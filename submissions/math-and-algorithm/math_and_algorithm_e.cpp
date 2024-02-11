#include <iostream>
using namespace std;
int main() {
    int N, sum = 0;
    cin >> N;
    for (int i = 0; i < N; ++i) {
        int a;
        cin >> a;
        sum += a;
    }
    cout << sum % 100 << endl;
}