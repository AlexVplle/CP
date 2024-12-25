#include <iostream>
#include <vector>
#include <queue>
#include <climits>
using namespace std;

struct Edge {
    int to, length;
};

struct State {
    int pos, usedBoots, time;
    bool operator<(const State &other) const {
        return time > other.time;  // Min-heap
    }
};

int dijkstra(int A, int B, int M, int L, int K, vector<vector<Edge>> &graph) {
    int start = 1, end = A + B;
    
    // Minimum time to reach each position with a specific number of boot uses
    vector<vector<int>> minTime(A + B + 1, vector<int>(K + 1, INT_MAX));
    priority_queue<State> pq;

    pq.push({start, 0, 0});
    minTime[start][0] = 0;

    while (!pq.empty()) {
        State curr = pq.top(); pq.pop();

        // If we reached the end
        if (curr.pos == end) return curr.time;

        // Ignore this state if not optimal
        if (curr.time > minTime[curr.pos][curr.usedBoots]) continue;

        for (auto &edge : graph[curr.pos]) {
            // Case 1: Walk normally
            int walkTime = curr.time + edge.length;
            if (walkTime < minTime[edge.to][curr.usedBoots]) {
                minTime[edge.to][curr.usedBoots] = walkTime;
                pq.push({edge.to, curr.usedBoots, walkTime});
            }

            // Case 2: Use the magic boot (only between villages)
            if (curr.usedBoots < K && curr.pos <= A && edge.to <= A && edge.length <= L) {
                if (curr.time < minTime[edge.to][curr.usedBoots + 1]) {
                    minTime[edge.to][curr.usedBoots + 1] = curr.time;
                    pq.push({edge.to, curr.usedBoots + 1, curr.time});
                }
            }
        }
    }
    return -1;  // If unreachable (shouldn't happen).
}

int main() {
    int T;
    cin >> T;
    while (T--) {
        int A, B, M, L, K;
        cin >> A >> B >> M >> L >> K;

        vector<vector<Edge>> graph(A + B + 1);

        for (int i = 0; i < M; ++i) {
            int X, Y, Li;
            cin >> X >> Y >> Li;
            graph[X].push_back({Y, Li});
            graph[Y].push_back({X, Li});
        }

        int result = dijkstra(A, B, M, L, K, graph);
        cout << result << endl;
    }
    return 0;
}

