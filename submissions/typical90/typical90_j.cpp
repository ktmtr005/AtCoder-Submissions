#include <bits/stdc++.h>
using namespace std;
const int INF = 200000;
int solve(vector<int> &num_1, vector<int> &num_2, vector<int> &score_1, vector<int> &score_2, int &ans1, int &ans2, int &L, int &R) {
    int l1, r1, l2, r2;
    l1 = *prev(lower_bound(num_1.begin(), num_1.end(), L));
    l2 = *prev(lower_bound(num_2.begin(), num_2.end(), L));
    r1 = *prev(upper_bound(num_1.begin(), num_1.end(), R));
    r2 = *prev(upper_bound(num_2.begin(), num_2.end(), R));
    if (l1 == r1)
        ans1 = 0;
    else
        ans1 = score_1[r1] - score_1[l1];
    if (l2 == r2)
        ans2 = 0;
    else
        ans2 = score_2[r2] - score_2[l2];
    return 0;
}
int main() {
    // 1. 入力
    int N;
    cin >> N;
    int C, P;
    vector<int> num_1(N + 2, INF);    // 学籍番号 (クラス1)
    vector<int> num_2(N + 2, INF);    // 学籍番号 (クラス2)
    vector<int> score_1(N + 2, INF);  // 累積スコア (クラス1)
    vector<int> score_2(N + 2, INF);  // 累積スコア (クラス2)
    int pre_P_1 = 0;
    int pre_P_2 = 0;
    num_1.at(0) = 0;
    num_2.at(0) = 0;
    score_1.at(0) = 0;
    score_2.at(0) = 0;
    int cnt1 = 0, cnt2 = 0;
    for (int i = 1; i <= N; i++) {
        cin >> C >> P;
        if (C == 1) {
            cnt1++;
            score_1.at(i) = pre_P_1 + P;
            num_1.at(cnt1) = i;
            pre_P_1 += P;
        } else {
            cnt2++;
            score_2.at(i) = pre_P_2 + P;
            num_2.at(cnt2) = i;
            pre_P_2 += P;
        }
    }
    num_1.push_back(INF);
    num_2.push_back(INF);
    int Q;
    cin >> Q;
    for (int i = 0; i < Q; i++) {
        int L, R;
        int ans1, ans2;
        cin >> L >> R;
        solve(num_1, num_2, score_1, score_2, ans1, ans2, L, R);
        cout << ans1 << ' ' << ans2 << endl;
    }
}