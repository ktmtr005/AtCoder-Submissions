#include <iostream>
#include <vector>
using namespace std;
int main() {
    vector<long long> nums(64, 1);
    for (auto &i : nums) cin >> i;
    uint64_t ans = 0;
    for (int i = 0; i < 64; i++) {
        ans += (nums[i] << i);
    }
    cout << ans << endl;
}