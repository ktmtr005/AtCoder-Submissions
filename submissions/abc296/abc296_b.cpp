#include <bits/stdc++.h>
using namespace std;
int main() {
    string row = "12345678";
    string column = "abcdefgh";
    vector<vector<bool>> S(8, vector<bool>(8));
    // 1. 入力
    for (int i = 0; i < 8; i++) {
        for (int j = 0; j < 8; j++) {
            char s;
            cin >> s;
            if (s == '*')
                S[7 - i][j] = true;
            else
                S[7 - i][j] = false;
        }
    }
    for (int i = 0; i < 8; i++) {
        for (int j = 0; j < 8; j++) {
            if (S[i][j] == true) {