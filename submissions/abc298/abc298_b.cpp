#include <bits/stdc++.h>
using namespace std;
int main() {
    int N;
    cin >> N;
    vector<vector<int>> A(N, vector<int>(N));
    vector<vector<int>> B(N, vector<int>(N));
    vector<vector<int>> rot_90(N, vector<int>(N));
    vector<vector<int>> rot_180(N, vector<int>(N));
    vector<vector<int>> rot_270(N, vector<int>(N));
    for (auto &i : A) {
        for (int &j : i) cin >> j;
    }
    for (auto &i : B) {
        for (int &j : i) cin >> j;
    }
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            rot_90[i][j] = A[N - 1 - j][i];
            rot_180[i][j] = rot_90[N - 1 - j][i];
            rot_270[i][j] = rot_180[N - 1 - j][i];
        }
    }
    bool ans = true, ans_90 = true, ans_180 = true, ans_270 = true;
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            if (A[i][j] == 1 && B[i][j] == 0) ans = false;
            if (rot_90[i][j] == 1 && B[i][j] == 0) ans_90 = false;
            if (rot_180[i][j] == 1 && B[i][j] == 0) ans_180 = false;
            if (rot_270[i][j] == 1 && B[i][j] == 0) ans_270 = false;
        }
    }
    if (ans || ans_90 || ans_180 || ans_270)
        cout << "Yes" << endl;
    else
        cout << "No" << endl;
}