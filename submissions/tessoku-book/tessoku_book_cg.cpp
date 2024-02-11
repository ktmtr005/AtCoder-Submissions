#include <iostream>
#include <vector>
using namespace std;
int main() {
    int N;
    cin >> N;
    vector<vector<int>> g(1501, vector<int>(1501, 0));
    for (int i = 0; i < N; i++) {
        int x, y;
        cin >> x >> y;
        g[y][x] += 1;
    }
    // 横方向の累積和
    vector<vector<int>> s(1501, vector<int>(1501, 0));
    for (int y = 1; y <= 1500; y++) {
        for (int x = 1; x <= 1500; x++) {
            s[y][x] = s[y][x - 1] + g[y][x];
        }
    }
    // 縦方向の累積和
    for (int x = 1; x <= 1500; x++) {
        for (int y = 1; y <= 1500; y++) {
            s[y][x] = s[y - 1][x] + s[y][x];
        }
    }
    int Q;
    cin >> Q;
    for (int i = 0; i < Q; i++) {
        int a, b, c, d, ans;
        cin >> a >> b >> c >> d;
        ans = s[d][c] + s[b - 1][a - 1] - s[d][a - 1] - s[b - 1][c];
        cout << ans << endl;
    }
    return 0;
}