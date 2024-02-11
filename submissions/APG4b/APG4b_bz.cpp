#include <bits/stdc++.h>
using namespace std;
int main() {
    int N;
    cin >> N;
    map<int, int> A;
    vector<int> a(N);
    for (int &i : a) {
        cin >> i;
    }
    for (int i : a) {
        if (A.count(i)) {
            A.at(i) += 1;
        } else
            A[i] = 1;
    }
    int max_cnt, max_val;
    max_cnt = 0;
    max_val = -1;
    for (auto [i, j] : A) {
        if (j > max_cnt) {
            max_cnt = j;
            max_val = i;
        }
    }
    cout << max_val << ' ' << max_cnt << endl;
}