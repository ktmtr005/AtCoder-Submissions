#include <bits/stdc++.h>
using namespace std;
int main() {
    int N;
    cin >> N;
    int64_t L0 = 2, L1 = 1;
    int64_t Li = 0;
    if (N == 1) Li = 1;
    for (int i = 2; i <= N; i++) {
        Li = L0 + L1;
        L0 = L1;
        L1 = Li;
    }
    cout << Li << endl;
}