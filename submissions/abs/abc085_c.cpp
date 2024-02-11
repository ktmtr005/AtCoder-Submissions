#include <bits/stdc++.h>
using namespace std;
int main() {
    int N, Y;
    cin >> N >> Y;
    int res10000 = -1, res5000 = -1, res1000 = -1;
    for (int i = 0; i <= N; ++i) {
        for (int j = 0; i + j <= N; ++j) {
            int k = N - i - j;
            int total = 10000 * i + 5000 * j + 1000 * k;
            if (total == Y) {
                res10000 = i;
                res5000 = j;
                res1000 = k;
            }
        }
    }
    cout << res10000 << ' ' << res5000 << ' ' << res1000 << endl;
}