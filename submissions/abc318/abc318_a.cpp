#include <iostream>
using namespace std;
int main() {
    int N, M, P;
    cin >> N >> M >> P;
    int a = N - M;
    int ans;
    if (N < M)
        ans = 0;
    else
        ans = a / P + 1;
    cout << ans << endl;
}