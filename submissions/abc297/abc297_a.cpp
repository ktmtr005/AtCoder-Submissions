#include <bits/stdc++.h>
using namespace std;
int main() {
    int N, D;
    cin >> N >> D;
    vector<int> T(N);
    for (int &i : T) cin >> i;
    int time = -1;
    for (int i = 1; i < N; i++) {
        if (T[i] - T[i - 1] <= D) {
            time = T[i];
            break;
        }
    }
    cout << time << endl;
}