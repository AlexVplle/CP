#include <iostream>
#include <cstring>
#include <algorithm>
#include <cmath>
#include <iomanip>

#define int long long

int x[1000], y[1000];
double dp[1000][1000], dist[1000][1000];
int N;

double distance(int a, int b) {
    return hypot(x[a]-x[b], y[a] - y[b]);
}

double recur(int a, int b) {
    if (dp[a][b]!=-1.0) return dp[a][b];
    if (a==N-1) return dp[a][b] = dp[b][a] = dist[b][N-1];
    return dp[a][b] = dp[b][a] = std::min(recur(a+1,b) + dist[a][a+1] , recur(a+1, a) + dist[b][a+1]);
}

signed main()
{
    std::cout << std::fixed << std::setprecision(2);
    while(std::cin >> N) {
        for (int i = 0 ; i < N; i++)
            for (int j = 0 ; j < N; j++)
                dp[i][j] = -1.0;
        for (int i = 0,a,b ; i < N; i++)
            std::cin >> x[i] >> y[i];
        for (int i = 0; i < N - 1; i++)
            for (int j = i + 1; j < N; j++)
                dist[i][j] = dist[j][i] = distance(i,j); std::cout << recur(0,0) << std::endl;
    }
}
