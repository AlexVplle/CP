#include <algorithm>
#include <iostream>
#include <unordered_map>
#include <vector>

#define int long long

signed main() {
    std::cin.tie(0)->sync_with_stdio(0);
    std::unordered_map<int, int> lookup;
    int a, b;
    while (std::cin >> a >> b) {
        int max_cycle_length = 0;
		int start = std::min(a, b);
		int end = std::max(a, b);
        for (size_t i = start; i <= end; i++) {
            std::vector<int> prev_number;
            int cycle_length = 1;
            int n = i;
            while (n != 1) {
                if (lookup.count(n) != 0) {
                    cycle_length += lookup[n] - 1;
                    break;
                }
                prev_number.push_back(n);
                if (n % 2) {
                    n = 3 * n + 1;
                } else {
                    n /= 2;
                }
                cycle_length++;
            }
            for (size_t j = 0; j < prev_number.size(); j++) {
                lookup[prev_number[j]] = cycle_length - j;
        	}
            max_cycle_length = std::max(max_cycle_length, cycle_length);
        }
        std::cout << a << " " << b << " " << max_cycle_length << std::endl;
    }
    return 0;
}

