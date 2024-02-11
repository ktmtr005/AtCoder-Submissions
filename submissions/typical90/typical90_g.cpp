#include <bits/stdc++.h>
using namespace std;
int main() {
    // 1. 入力
    int N, Q;
    cin >> N;
    vector<int> A(N, 0);
    for (int &i : A) {
        cin >> i;
    }
    cin >> Q;
    vector<int> B(Q, 0);
    for (int &i : B) {
        cin >> i;
    }
    // 2. ソート
    sort(A.begin(), A.end());