#include <cstdio>
#include <iostream>

int main (int argc, char *argv[]) {
	int n, a, b;
	std::cin >> n;
	for (size_t i = 0; i < n; i++) {
		std::cin >> a >> b;
		if (a < b) {
			std::cout << "<";
		} else if (a == b) {
			std::cout << "=";
		} else {
			std::cout << ">";
		}
		std::cout << std::endl;
	}
	return 0;
}
