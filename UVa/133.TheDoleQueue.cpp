#include <iostream>
#include <vector>
#include <iomanip>

#define int long long

signed main() {
    int N, k, m;
    while (std::cin >> N >> k >> m) {
        if (N == 0 && k == 0 && m == 0) break;
		std::vector<int> applicants(N);
		for (int i = 0; i < N; ++i) {
			applicants[i] = i + 1;
		}

		int left = 0;
		int right = N - 1;
		int remaining = N;
		bool firstOutput = true;

		while (remaining > 0) {
			int steps = k;
			while (steps > 0) {
				if (applicants[left] != 0) {
					steps--;
					if (steps == 0) break;
				}
				left = (left + 1) % N;
			}
			steps = m;
			while (steps > 0) {
				if (applicants[right] != 0) {
					steps--;
					if (steps == 0) break;
				}
				right = (right - 1 + N) % N;
			}
			if (!firstOutput) {
				std::cout << ",";
			}

			if (left == right) {
				std::cout << std::setw(3) << applicants[left];
				applicants[left] = 0;
				remaining--;
			} else {
				std::cout << std::setw(3) << applicants[left] << std::setw(3) << applicants[right];
				applicants[left] = applicants[right] = 0;
				remaining -= 2;
			}
			left = (left + 1) % N;
			right = (right - 1 + N) % N;
			firstOutput = false;
		}
		std::cout << std::endl;
    }
    return 0;
}

