#include <algorithm>
#include <iostream>
#include <iomanip>
#include <vector>
#include <cmath>

#define int long long

signed main() {
	std::cin.tie(0)->sync_with_stdio(0);
    int n;
    while (std::cin >> n, n != 0) {
        std::vector<int> expenses(n);
        int total = 0;
        for (int i = 0; i < n; ++i) {
            double expense;
            std::cin >> expense;
            expenses[i] = static_cast<int>(round(expense * 100));
            total += expenses[i];
        }
        int average = total / n;
		int remainder = total % n;
        int positive_diff = 0;
        int negative_diff = 0;
        std::sort(expenses.begin(), expenses.end());
        for (int i = 0; i < n; ++i) {
            int diff;
			if (i < n - remainder) {
            	diff = expenses[i] - average;
			} else {
				diff = expenses[i] - average + 1;
			}
            if (diff > 0) {
                positive_diff += diff;
            } else {
                negative_diff -= diff;
            }
        }
        std::cout << std::fixed << std::setprecision(2) << "$" << std::max(positive_diff, negative_diff) / 100.0 << std::endl;
    }
    return 0;
}

