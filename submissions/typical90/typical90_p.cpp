#include <bits/stdc++.h>
using namespace std;
int main() {
    int64_t A, B, C, N;
    cin >> N;
    cin >> A >> B >> C;
    int64_t ans = 10000;
    for (int i = 0; i < 10000; i++) {
        for (int j = 0; j < 10000; j++) {
            int64_t tmp = A * i + B * j;
            if ((N - tmp) % C != 0 || tmp > N) continue;
            int64_t tmp_ans = i + j + ((N - tmp) / C);
            if (tmp_ans < ans) ans = tmp_ans;
        }
    }
    cout << ans << endl;