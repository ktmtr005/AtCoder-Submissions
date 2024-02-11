#include <bits/stdc++.h>
using namespace std;
int main() {
    int R, C;
    cin >> R >> C;
    vector<vector<char>> B(R, vector<char>(C));
    for (int i = 0; i < R; i++) {
        for (int j = 0; j < C; j++) {
            cin >> B.at(i).at(j);
        }
    }
    vector<vector<bool>> blasted(R, vector<bool>(C));
    for (int i = 0; i < R; i++) {
        for (int j = 0; j < C; j++) {
            if (!isdigit(B[i][j])) continue;
            int power = B[i][j] - '0';
            for (int ni = 0; ni < R; ni++) {
                for (int nj = 0; nj < C; nj++) {