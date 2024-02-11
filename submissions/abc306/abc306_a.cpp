#include <iostream>
using namespace std;
int main() {
    int N;
    string S;
    cin >> N;
    cin >> S;
    for (char &i : S) {
        printf("%c%c", i, i);
    }
    printf("\n");
}