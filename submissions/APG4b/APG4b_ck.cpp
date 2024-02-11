#include <bits/stdc++.h>
using namespace std;
int main() {
    string S;
    int x = 1;
    cin >> S;
    for (int i = 1; i < S.size(); i += 2) {
        if (S[i] == '+') ++x;
        if (S[i] == '-') --x;
    }
    cout << x << endl;
}