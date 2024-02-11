#include <bits/stdc++.h>
using namespace std;
int main() {
    // 1. 入力
    int H, W;
    cin >> H >> W;
    vector<vector<int>> A(H, vector<int>(W));
    // 2. 行、列ごとの合計を計算
    vector<int> sum_row(H, 0);
    vector<int> sum_column(W, 0);
    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            cin >> A.at(i).at(j);
            sum_row[i] += A[i][j];
            sum_column[j] += A[i][j];
        }
    }
    // 3. 出力
    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            int value = sum_row[i] + sum_column[j] - A[i][j];
            cout << value;
            if (j == W - 1)
                cout << endl;
            else
                cout << ' ';
        }
    }
}