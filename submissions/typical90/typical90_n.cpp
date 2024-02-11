#include <bits/stdc++.h>
using namespace std;
int main() {
    // 1. 入力
    int N;
    cin >> N;
    vector<int> A(N);
    vector<int> B(N);
    for (int &i : A) cin >> i;
    for (int &i : B) cin >> i;
    // 2. 距離の計算
    sort(A.begin(), A.end());
    sort(B.begin(), B.end());
    int64_t dist = 0;
    for (int i = 0; i < N; i++) {
        dist += abs(A[i] - B[i]);
    }
    // 3. 出力
    cout << dist << endl;
}