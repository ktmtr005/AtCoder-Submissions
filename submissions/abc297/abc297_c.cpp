#include <bits/stdc++.h>
using namespace std;
int main() {
    int H, W;
    cin >> H >> W;
    vector<string> S(H);
    for (string &i : S) cin >> i;
    for (string &i : S) {
        for (int j = 0; j < W - 1; j++) {
            if (i[j] == 'T' && i[j + 1] == 'T') i.replace(j, 2, "PC");
        }
    }
    for (string &i : S) cout << i << endl;
}