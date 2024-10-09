#include <iostream>
#include <unordered_map>

#define int long long

signed main() {
	std::cin.tie(0)->sync_with_stdio(0);
	std::unordered_map<int, int> lookup;
	int a, b;
	while (std::cin >> a >> b) {
		int max = 0;
		for (; a <= b; a++) {
			int cycle_length = 0;
			int value = a;
			while (value != 1) {
				if (lookup.count(value) != 0) {
					cycle_length += lookup[value];
					break;
				}
			}
		}
		std::cout << max << std::endl;
	}
	return 0;
}
