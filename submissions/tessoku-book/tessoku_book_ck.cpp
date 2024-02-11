#include <cmath>
#include <iostream>
using namespace std;
int main() {
    int N;
    cin >> N;
    double L = 0, R = N;
    while (L < R) {
        double M = (L + R) / 2;
        double sol = pow(M, 3) + M;
        if (abs(N - sol) <= 0.0001) {
            cout << M << endl;
            break;
        } else if (N < sol) {
            R = M;
        } else if (N > sol) {
            L = M;
        }
    }
}