#include <bits/stdc++.h>
using namespace std;
int main() {
    int N, A, B;
    char op;
    cin >> N >> A;
    for (int i = 0; i < N; i++) {
        cin >> op >> B;
        if (op == '+') {
            A += B;
        }
        if (op == '-') {
            A -= B;
        }
        if (op == '*') {
            A *= B;
        }
        if (op == '/' && B != 0) {