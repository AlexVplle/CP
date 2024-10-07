#include <algorithm>
#include <array>
#include <iostream>

int main (int argc, char *argv[]) {
	int n;
	std::array<int, 3> salaries;
	std::cin >> n;
	for (size_t i = 0; i < n; i++) {
		for (size_t j = 0; j < 3; j++) {
			std::cin >> salaries[j];
		}
		std::sort(salaries.begin(), salaries.end());
		std::cout << "Case " << i + 1 << ": " << salaries[1] << std::endl;
	}
	return 0;
}
