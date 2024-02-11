#include <bits/stdc++.h>
using namespace std;
struct Clock {
    int hour;
    int minute;
    int second;
    void set(int h, int m, int s) {
        hour = h;
        minute = m;
        second = s;
    }
    string to_str() {
        ostringstream ss_h, ss_m, ss_s;
        ss_h << setw(2) << setfill('0') << hour << ":";
        ss_m << setw(2) << setfill('0') << minute << ":";
        ss_s << setw(2) << setfill('0') << second;
        string time_str = ss_h.str() + ss_m.str() + ss_s.str();
        return time_str;
    }
    void shift(int diff_second) {
        int time_second = second + 60 * minute + 3600 * hour;
        time_second += diff_second;
        time_second %= 86400;
        if (time_second < 0) {
            time_second = 86400 + time_second;
        }
        second = time_second % 60;
        hour = time_second / 3600;
        minute = (time_second - second - 3600 * hour) / 60;
    }
};
int main() {
    int hour, minute, second;
    cin >> hour >> minute >> second;
    int diff_second;
    cin >> diff_second;
    Clock clock;
    clock.set(hour, minute, second);
    cout << clock.to_str() << endl;
    clock.shift(diff_second);
    cout << clock.to_str() << endl;
}