#include <iostream>
#include <vector>
using namespace std;
int main() {
    int T;
    cin >> T;
    for (int i = 0; i < T; i++) {
        int N;
        cin >> N;
        vector<int> P(N), d(N);
        for (int i = 0; i < N; i++) {
            cin >> P[i];
            d[i] = i + 1 - P[i];
        }
        int cnt = 0;
        for (int j = 0; j < N; j++) {
            if (j + 1 >= P[j]) {
                cnt++;
            }
            for (int k = j - 1; k >= 0; k--) {
                if (d[k] < d[j] && d[k] >= 0 && P[j] < P[k]) {
                    cnt--;
                    d[k] = -1;
                }
            }
        }
        if (cnt == 0) cnt = N;
        cout << cnt << endl;
    }
}