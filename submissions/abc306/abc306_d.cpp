#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
int main() {
    int N;
    cin >> N;
    int64_t dp[300005][2];
    vector<int64_t> X(N), Y(N);
    for (int i = 0; i <= N; i++) {
        dp[i][0] = INT64_MIN;
        dp[i][1] = INT64_MIN;
    }
    dp[0][0] = 0;
    for (int i = 0; i < N; i++) {
        cin >> X[i] >> Y[i];
    }
    for (int i = 0; i < N; i++) {
        if (X[i] == 0) {
            dp[i + 1][0] = max(dp[i][0], max(dp[i][0], dp[i][1]) + Y[i]);
        } else {
            dp[i + 1][1] = max(dp[i][1], dp[i][0] + Y[i]);
        }
        dp[i + 1][0] = max(dp[i + 1][0], dp[i][0]);
        dp[i + 1][1] = max(dp[i + 1][1], dp[i][1]);
    }
    std::cout << max(dp[N][0], dp[N][1]) << "\n";
}