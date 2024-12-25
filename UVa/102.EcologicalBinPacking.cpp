#include <array>
#include <iostream>
#include <string>

#define int long long

signed main() {
	std::array<std::string, 6> order = {"BCG", "BGC", "CBG", "CGB", "GBC", "GCB"};
	std::array<int, 9> in;
    while (std::cin >> in[0] >> in[1] >> in[2] >> in[3] >> in[4] >> in[5] >> in[6] >> in[7] >> in[8])
    {
        int solutions[6];
        solutions[0] = in[1] + in[2] + in[3] + in[4] + in[6] + in[8];
        solutions[1] = in[1] + in[2] + in[3] + in[5] + in[6] + in[7];
        solutions[2] = in[0] + in[1] + in[4] + in[5] + in[6] + in[8];
        solutions[3] = in[0] + in[1] + in[3] + in[5] + in[7] + in[8];
        solutions[4] = in[0] + in[2] + in[4] + in[5] + in[6] + in[7];
        solutions[5] = in[0] + in[2] + in[3] + in[4] + in[7] + in[8];
        int min = solutions[0];
        int index = 0;
        for (int i = 1; i < 6; i++)
        {
            if (solutions[i] < min)
            {
                min = solutions[i];
                index = i;
            }
        }
		std::cout << order[index] << " " << solutions[index] << std::endl;
    }
}
