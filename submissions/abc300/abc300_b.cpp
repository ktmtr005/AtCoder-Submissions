#include <bits/stdc++.h>
using namespace std;
int main() {
    int H, W;
    cin >> H >> W;
    vector<string> A(H), B(H);
    for (auto &i : A) {
        cin >> i;
    }
    for (auto &i : B) {
        cin >> i;
    }
    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            if (A == B) {
                cout << "Yes" << endl;
                goto end;
            }
            for (auto &k : A) {
                rotate(k.rbegin(), k.rbegin() + 1, k.rend());
            }
        }
        rotate(A.rbegin(), A.rbegin() + 1, A.rend());
    }
    cout << "No" << endl;
end:
    return 0;
}