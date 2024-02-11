#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
int main() {
    int N;
    cin >> N;
    vector<int> A(N);
    for (auto &i : A) {
        cin >> i;
    }
    sort(A.begin(), A.end());
    for (int i = 1; i < N; ++i) {
        if (A[i] - A[i - 1] != 1) {
            cout << A[i] - 1 << endl;
            break;
        }
    }
}