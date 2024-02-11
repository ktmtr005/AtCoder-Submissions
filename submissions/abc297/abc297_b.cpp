#include <bits/stdc++.h>
using namespace std;
int main() {
    string S;
    cin >> S;
    bool ans = false;
    int K_pos, B_pos1 = -1, B_pos2 = -1, R_pos1 = -1, R_pos2 = -1;
    for (int i = 0; i < 8; i++) {
        if (S[i] == 'B' && B_pos1 == -1) B_pos1 = i;
        if (S[i] == 'B' && B_pos1 != -1) B_pos2 = i;
        if (S[i] == 'K') K_pos = i;
        if (S[i] == 'R' && R_pos1 == -1) R_pos1 = i;
        if (S[i] == 'R' && R_pos1 != -1) R_pos2 = i;
    }
    if (R_pos1 < K_pos && K_pos < R_pos2 && B_pos1 % 2 != B_pos2 % 2) ans = true;
    if (ans == true)
        cout << "Yes" << endl;
    else
        cout << "No" << endl;