#include <bits/stdc++.h>
#define REP(i, n) for (int i = 0; (i) < (int)(n); ++(i))
#define REP3(i, m, n) for (int i = (m); (i) < (int)(n); ++(i))
#define REP_R(i, n) for (int i = (int)(n)-1; (i) >= 0; --(i))
#define REP3R(i, m, n) for (int i = (int)(n)-1; (i) >= (int)(m); --(i))
#define ALL(x) ::std::begin(x), ::std::end(x)
using namespace std;
deque<pair<int, int>> d;
int query1(char C) {
    auto [x, y] = d[0];
    if (C == 'R') d.push_front({x + 1, y});
    if (C == 'L') d.push_front({x - 1, y});
    if (C == 'U') d.push_front({x, y + 1});
    if (C == 'D') d.push_front({x, y - 1});
    d.pop_back();
    return 0;
}
pair<int, int> query2(int p) {
    pair<int, int> out = d.at(p - 1);
    return out;
}
int main() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);
    int N, Q;
    cin >> N >> Q;
    for (int i = 1; i <= N; i++) d.push_back({i, 0});
    while (Q--) {
        int t;
        cin >> t;
        if (t == 1) {
            char C;
            cin >> C;
            query1(C);
        } else {
            int p;
            cin >> p;
            pair<int, int> xy = query2(p);
            cout << xy.first << ' ' << xy.second << "\n";
        }
    }
    return 0;
}