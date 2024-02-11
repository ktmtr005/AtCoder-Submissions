#include <bits/stdc++.h>
using namespace std;
int N;
string generate_string(int bit) {
    // bit -> string
    string s;
    for (int i = 1; i < (1 << N); i <<= 1) {
        if (bit & i)
            s.push_back(')');
        else
            s.push_back('(');
    }
    reverse(s.begin(), s.end());
    return s;
}
int main() {
    cin >> N;
    // ビット全探索
    for (int bit = 0; bit < (1 << N); bit++) {