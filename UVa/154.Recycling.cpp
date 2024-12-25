#include <cstddef>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

#define int long long

std::unordered_map<char, std::unordered_map<char, std::size_t>> create_hashmap() {
	return {
		{'r', {
		 {'P', 0},
		 {'G', 0},
		 {'A', 0},
		 {'S', 0},
		 {'N', 0}
		 }},
		{'o', {
		 {'P', 0},
		 {'G', 0},
		 {'A', 0},
		 {'S', 0},
		 {'N', 0}
		 }},
		{'y', {
		 {'P', 0},
		 {'G', 0},
		 {'A', 0},
		 {'S', 0},
		 {'N', 0}
		 }},
		{'g', {
		 {'P', 0},
		 {'G', 0},
		 {'A', 0},
		 {'S', 0},
		 {'N', 0}
		 }},
		{'b', {
		 {'P', 0},
		 {'G', 0},
		 {'A', 0},
		 {'S', 0},
		 {'N', 0}
		 }},
	};
}

signed main() {
	std::string line;
	std::cin >> line;
	std::vector<std::string> block;
	std::unordered_map<char, std::unordered_map<char, std::size_t>> hashmap = create_hashmap();
	while (line != "#") {
		if (line[0] == 'e') {
			size_t best_city = 0;
			size_t max_combinaison = 0;
			for (size_t i = 0; i < block.size(); i++) {
				size_t combinaison = 0;
				line = block[i];
				for (size_t j = 0; j < line.length(); j += 4) {
					combinaison += hashmap[line[j]][line[j + 2]];
				}
				if (combinaison > max_combinaison) {
					max_combinaison = combinaison;
					best_city = i + 1;
				}
			}
			std::cout << best_city << std::endl;
			hashmap = create_hashmap();
			block.clear();
		} else {
			for (size_t i = 0; i < line.length(); i += 4) {
				hashmap[line[i]][line[i + 2]]++;
			}
			block.push_back(line);
		}
		std::cin >> line;
	}
	return 0;
}


