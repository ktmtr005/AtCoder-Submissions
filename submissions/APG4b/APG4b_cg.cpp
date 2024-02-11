#include <bits/stdc++.h>
using namespace std;
int main() {
    vector<int> a(5);
    for (int i = 0; i < 5; i++) {
        cin >> a.at(i);
    }
    bool exist = false;
    for (int i = 0; i < 4; i++) {
        if (a.at(i) == a.at(i + 1)) {
            exist = true;
            break;
        }
    }
    if (exist)
        cout << "YES" << endl;
    else
        cout << "NO" << endl;
}