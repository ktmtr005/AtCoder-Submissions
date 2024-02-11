#include <bits/stdc++.h>
using namespace std;
int main() {
    int N, X;
    cin >> N >> X;
    vector<int> A(N);
    for (int &a : A) {
        cin >> a;
    }
    sort(A.begin(), A.end());
    bool exist = false;
    for (int i = 0; i < N; i++) {
        int tmp = X + A[i];
        if (binary_search(A.begin(), A.end(), tmp) == true) {
            exist = true;
            break;
        }
    }
    if (exist == true)
        cout << "Yes" << endl;
    else
        cout << "No" << endl;
}