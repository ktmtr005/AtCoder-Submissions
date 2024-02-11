#include <bits/stdc++.h>
using namespace std;
int main() {
    int N, c = 0, tmp = 0;
    cin >> N;
    vector<int> d(N);
    for (int i = 0; i < N; i++) {
        cin >> d.at(i);
    }
    sort(d.begin(), d.end());
    for (int i = 0; i < N; i++) {
        if (d.at(i) != tmp) {
            c++;
        }
        tmp = d.at(i);
    }
    cout << c << endl;
}