#include <bits/stdc++.h>
using namespace std;
int main() {
    int N;
    cin >> N;
    vector<pair<int, int>> ab(N, pair<int, int>(0, 0));
    for (auto &i : ab) {
        int a, b;
        cin >> a >> b;
        i.first = b;
        i.second = a;
    }
    sort(ab.begin(), ab.end());
    for (auto &i : ab) {
        cout << i.second << ' ' << i.first << endl;
    }
}