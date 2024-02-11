#include <iostream>
#include <vector>
using namespace std;
const int INF = 2'000'000'000;
int main() {
    int N;
    cin >> N;
    vector<int> h(N + 1);
    for (int i = 1; i <= N; i++) {
        cin >> h[i];
    }
    vector<int> dp(N + 2, INF);
    dp[1] = 0;
    for (int i = 1; i < N; i++) {
        dp[i + 1] = min(dp[i + 1], dp[i] + abs(h[i + 1] - h[i]));
        dp[i + 2] = min(dp[i + 2], dp[i] + abs(h[i + 2] - h[i]));
    }
    cout << dp[N] << endl;
}