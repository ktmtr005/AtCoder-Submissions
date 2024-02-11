#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
int main() {
    // 入力
    int N;
    cin >> N;
    vector<int> A(N + 1), B(N + 1);
    for (int i = 2; i <= N; i++) {
        cin >> A[i];
    }
    for (int i = 3; i <= N; i++) {
        cin >> B[i];
    }
    // 動的計画法
    vector<int> dp(N + 1);
    dp[1] = 0;
    dp[2] = A[2];
    for (int i = 3; i <= N; i++) {
        dp[i] = min(dp[i - 1] + A[i], dp[i - 2] + B[i]);
    }
    // 最短経路を復元
    vector<int> ans;
    int place = N;
    while (true) {
        ans.push_back(place);
        if (place == 1) break;
        if (dp[place - 1] + A[place] == dp[place]) {
            place = place - 1;
        } else {
            place = place - 2;
        }
    }
    reverse(ans.begin(), ans.end());
    // 答えを出力
    cout << ans.size() << endl;
    for (int i = 0; i < ans.size(); i++) {
        if (i >= 1) {
            cout << ' ';
        }
        cout << ans[i];
    }
    cout << endl;
}