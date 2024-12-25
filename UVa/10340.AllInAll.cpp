#include <cstddef>
#include <iostream>
#include <string>

#define int long long

signed main() {
	std::string sequence, word;
	while (std::cin >> sequence >> word) {
		size_t cursor_sequence = 0;
		for (size_t i = 0; i < word.length(); i++) {
			if (word[i] == sequence[cursor_sequence]) {
				cursor_sequence++;
			}
		}
		if (cursor_sequence == sequence.length()) {
			std::cout << "yes";
		} else {
			std::cout << "no";
		}
		std::cout << std::endl;
	}
	return 0;
}
