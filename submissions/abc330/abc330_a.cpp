#include <bits/stdc++.h>
#define REP(i, n) for (int i = 0; (i) < (int)(n); ++(i))
#define REP3(i, m, n) for (int i = (m); (i) < (int)(n); ++(i))
#define REP_R(i, n) for (int i = (int)(n)-1; (i) >= 0; --(i))
#define REP3R(i, m, n) for (int i = (int)(n)-1; (i) >= (int)(m); --(i))
#define ALL(x) ::std::begin(x), ::std::end(x)
using namespace std;
int64_t solve(int N, int64_t L, const std::vector<int64_t> &A) {
    // TODO: edit here
    int64_t passed = 0;
    for (auto &i : A) {
        if (i >= L) passed += 1;
    }
    return passed;
}
// generated by oj-template v4.8.1 (https://github.com/online-judge-tools/template-generator)
int main() {