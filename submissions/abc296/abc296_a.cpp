#include <bits/stdc++.h>
using namespace std;
int main() {
    int N;
    string S;
    cin >> N;
    cin >> S;
    bool result = true;
    for (int i = 1; i < N; i++) {
        if (S[i] == S[i - 1]) {
            result = false;
            break;
        }
    }
    if (result == true)
        cout << "Yes" << endl;
    else
        cout << "No" << endl;
}