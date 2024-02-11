#include <bits/stdc++.h>
using namespace std;
const int64_t INF = INT64_MAX;
int main() {
    int64_t N, M, x;
    cin >> N >> M;
    int64_t ans = INF;
    for (int64_t i = 1; i <= N; i++) {
        x = (M + i - 1) / i;
        if (x <= N) ans = min(ans, i * x);
        if (i > x) break;
    }
    if (ans == INF)
        cout << -1 << endl;
    else
        cout << ans << endl;
}