#include <iostream>
#include <numeric>
#include <vector>
using namespace std;
int main() {
    int N;
    cin >> N;
    vector<vector<int>> E(101, vector<int>(101));
    for (int i = 0; i < N; i++) {
        int a, b, c, d;
        cin >> a >> b >> c >> d;
        for (int x = a; x < b; x++) {
            for (int y = c; y < d; y++) {
                E[x][y] = 1;
            }
        }
    }
    int s = 0;
    for (auto &i : E) {
        s += accumulate(i.begin(), i.end(), 0);
    }
    cout << s << endl;
}