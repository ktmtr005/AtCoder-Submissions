#include <iostream>
#include <vector>
using namespace std;
int N, M;
int ans = 0;
vector<bool> used(11, false);
vector<vector<int>> E(11, vector<int>(11, 0));
int dfs(int v, int sum) {
    used[v] = true;
    if (sum > ans) ans = sum;
    for (int i = 1; i <= N; ++i) {
        if (used[i] == false && E[v][i] != 0) {
            dfs(i, sum + E[v][i]);
        }
    }
    used[v] = false;
    return 0;
}
int main() {
    cin >> N >> M;
    for (int i = 0; i < M; ++i) {
        int a, b, c;
        cin >> a >> b >> c;
        E[a][b] = c;
        E[b][a] = c;
    }
    for (int i = 1; i <= N; ++i) {
        dfs(i, 0);
    }
    cout << ans << endl;
}