#include <iostream>
#include <vector>
#include <string>

#define int long long

const int digit_segments[10] = {
    0b0111111, // 0 
    0b0000110, // 1 
    0b1011011, // 2 
    0b1001111, // 3 
    0b1100110, // 4 
    0b1101101, // 5 
    0b1111101, // 6 
    0b0000111, // 7 
    0b1111111, // 8 
    0b1101111 // 9 
};

signed main() {
    int s;
	std::string n;

    while (std::cin >> s >> n) {
        if (s == 0 && n == "0") break;
		std::vector<std::string> output(2 * s + 3, "");
        for (size_t idx = 0; idx < n.size(); ++idx) {
            int digit = n[idx] - '0';
			std::vector<std::string> digit_display(2 * s + 3, std::string(s + 2, ' '));
			int segments = digit_segments[digit];
			if (segments & 0b0000001) {
				for (int i = 1; i <= s; ++i) {
					digit_display[0][i] = '-';
				}
			}
			if (segments & 0b0001000) {
				for (int i = 1; i <= s; ++i) {
					digit_display[2 * s + 2][i] = '-';
				}
			}
			if (segments & 0b1000000) {
				for (int i = 1; i <= s; ++i) {
					digit_display[s + 1][i] = '-';
				}
			}
			if (segments & 0b0100000) {
				for (int i = 1; i <= s; ++i) {
					digit_display[i][0] = '|';
				}
			}
			if (segments & 0b0000010) {
				for (int i = 1; i <= s; ++i) {
					digit_display[i][s + 1] = '|';
				}
			}
			if (segments & 0b0010000) {
				for (int i = s + 2; i <= 2 * s + 1; ++i) {
					digit_display[i][0] = '|';
				}
			}
			if (segments & 0b0000100) {
				for (int i = s + 2; i <= 2 * s + 1; ++i) {
					digit_display[i][s + 1] = '|';
				}
			}
            for (int i = 0; i < 2 * s + 3; ++i) {
                if (idx != 0) output[i] += " ";
                output[i] += digit_display[i];
            }
        }
        for (const std::string& line : output) {
            std::cout << line << std::endl;
        }
        std::cout << std::endl;
    }

    return 0;
}
