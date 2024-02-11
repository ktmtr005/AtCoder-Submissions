#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
int main() {
    int N;
    cin >> N;
    vector<int> A(3 * N, 0), f(N + 1, 0), cnt(N + 1, 0);
    for (auto &i : A) cin >> i;
    for (int i = 0; i < 3 * N; i++) {
        int a = A[i];
        if (cnt[a] == 1) {
            f[a] = i;
            cnt[a]++;
        } else
            cnt[a]++;
    }
    sort(f.begin(), f.end());
    for (int i = 1; i <= N; i++) {
        cout << A[f[i]];
        if (i != N) cout << ' ';
    }
    cout << endl;
}