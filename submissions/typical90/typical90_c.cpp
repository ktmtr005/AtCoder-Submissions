#include <bits/stdc++.h>
using namespace std;
// 入力
int N;
int A[1 << 18], B[1 << 18];
// グラフ
const int INF = (1 << 29);  // 無限大
vector<int> G[1 << 18];
int dist[1 << 18];
void getdist(int start) {
    // 最短距離を計算
    for (int i = 1; i <= N; i++) dist[i] = INF;
    queue<int> Q;
    Q.push(start);
    dist[start] = 0;
    while (!Q.empty()) {
        int pos = Q.front();
        Q.pop();
        for (int to : G[pos]) {
            if (dist[to] == INF) {
                dist[to] = dist[pos] + 1;
                Q.push(to);
            }
        }
    }
}
int main() {
    // 1. 入力
    cin >> N;
    for (int i = 1; i <= N - 1; i++) {
        cin >> A[i] >> B[i];
        G[A[i]].push_back(B[i]);
        G[B[i]].push_back(A[i]);
    }
    // 2. 頂点1からの最短距離を求める
    getdist(1);
    int maxn1 = -1, maxid1 = -1;  // maxid1: 頂点1から最も離れている頂点
    for (int i = 1; i <= N; i++) {
        if (maxn1 < dist[i]) {
            maxn1 = dist[i];
            maxid1 = i;
        }
    }
    // 3. 頂点maxid1からの最短距離を求める
    getdist(maxid1);
    int maxn2 = -1;
    for (int i = 1; i <= N; i++) {
        maxn2 = max(maxn2, dist[i]);
    }
    // 4. 出力
    cout << maxn2 + 1 << endl;
    return 0;
}