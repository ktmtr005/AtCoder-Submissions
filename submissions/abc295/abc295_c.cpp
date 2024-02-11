#include <bits/stdc++.h>
using namespace std;
int main() {
    int N;
    map<int, int> mp;
    cin >> N;
    for (int i = 0; i < N; i++) {
        int a;
        cin >> a;
        ++mp[a];
    }
    int ans = 0;
    for (auto [_, cnt] : mp) {
        ans += cnt / 2;
    }
    cout << ans << endl;
}