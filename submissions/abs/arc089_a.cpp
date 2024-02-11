#include <bits/stdc++.h>
using namespace std;
int main() {
    int N;
    cin >> N;
    vector<int> t(N + 1), x(N + 1), y(N + 1);
    for (int i = 0; i < N; i++) {
        cin >> t.at(i + 1) >> x.at(i + 1) >> y.at(i + 1);
    }
    bool can = true;
    for (int i = 0; i < N; i++) {
        int dt = t.at(i + 1) - t.at(i);
        int dist = abs(x.at(i + 1) - x.at(i)) + abs(y.at(i + 1) - y.at(i));
        if (dt < dist) {
            can = false;
        }
        if (dt % 2 != dist % 2) {
            can = false;
        }
    }
    if (can)
        cout << "Yes" << endl;
    else
        cout << "No" << endl;
}