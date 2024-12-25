#include <iostream>
#include <queue>
#include <string>
#include <utility>
#include <vector>
using ll = long long;

signed main () {
	std::cin.tie(0)->sync_with_stdio(0);
	ll number_of_words;
	while (std::cin >> number_of_words) {
		std::cout << "--" << std::endl;
		std::vector<std::string> words(number_of_words);
		for (size_t i = 0; i < number_of_words; i++) {
			std::cin >> words[i];
		}
		ll number_of_rules;
		std::string rule;
		std::cin >> number_of_rules;
		for (size_t i = 0; i < number_of_rules; i++) {
			std::cin >> rule;
			std::queue<std::pair<std::string, ll>> queue;
			queue.emplace("", 0);
			while (!queue.empty()) {
				auto [current, index] = queue.front();
				queue.pop();
				if (index == rule.length()) {
					std::cout << current << std::endl;
					continue;
				} else {
					if (rule[index] == '#') {
						for (const auto& word: words) {
							queue.emplace(current + word, index + 1);
						}
					} else {
						for (char i = '0'; i <= '9'; i++) {
							queue.emplace(current + i, index + 1);
						}
					}
				}
			}
		}
	}
	return 0;
}
