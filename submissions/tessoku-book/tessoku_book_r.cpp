#include <iostream>
#include <vector>
using namespace std;
int main() {
    int N, S;
    cin >> N >> S;
    vector<int> A(N + 1);
    for (int i = 1; i <= N; i++) {
        cin >> A[i];
    }
    vector<vector<bool>> dp(N + 1, vector<bool>(S + 1));
    // 動的計画法 (i=0)
    dp[0][0] = true;
    for (int i = 1; i <= N; i++) dp[0][i] = false;
    // 動的計画法 (i>=1)
    for (int i = 1; i <= N; i++) {
        for (int j = 0; j <= S; j++) {
            if (j < A[i]) {
                if (dp[i - 1][j] == true)
                    dp[i][j] = true;
                else
                    dp[i][j] = false;
            }
            if (j >= A[i]) {
                if (dp[i - 1][j] == true || dp[i - 1][j - A[i]] == true)
                    dp[i][j] = true;
                else
                    dp[i][j] = false;
            }
        }
    }
    if (dp[N][S] == true)
        cout << "Yes" << endl;
    else
        cout << "No" << endl;
}