#include <iostream>
#include <vector>
using namespace std;
int main() {
    int N;
    cin >> N;
    vector<int> A(N);
    for (auto &i : A) {
        cin >> i;
    }
    int ans = 0;
    for (auto &i : A) {
        ans += i;
    }
    cout << ans << endl;
}