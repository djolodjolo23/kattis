#include <iostream>
#include <vector>
// #include "Graph.hpp"
// #include "Bfs.hpp"
// #include "Dfs.hpp"
#include "Dsu.h"
using namespace std;

static const int dr[4] = { -1, 1, 0, 0 };
static const int dc[4] = {  0, 0, -1, 1 };

int main() {
    // int rows, cols;
    // cin >> rows >> cols;
    //
    // vector<string> matrix;
    // for (int i = 0; i < rows; i++) {
    //     string row;
    //     cin >> row;
    //     matrix.push_back(row);
    // }
    //
    // Graph graph(matrix);
    //
    // int numOfQueries;
    // cin >> numOfQueries;
    //
    // for (int i = 0; i < numOfQueries; i++) {
    //     int row_1, column_1, row_2, column_2;
    //     cin >> row_1 >> column_1 >> row_2 >> column_2;
    //
    //     Graph::Node* start = graph.getNode(row_1 - 1, column_1 - 1);
    //     Graph::Node* end = graph.getNode(row_2 - 1, column_2 - 1);
    //
    //     Bfs::bfs(start, end, rows, cols);
    //     //Dfs::dfs(start, end, rows, cols);
    // }
    vector<string> matrix = {
        "11111111111111111111",
        "11000000000000000101",
        "11111111111111110000",
        "11111111111111110000",
        "11000000000000000111",
        "00011111111111111111",
        "00111111111111111111",
        "10000000000000001111",
        "11111111111111111111",
        "11111111111111111111"
    };

    int row_1 = 8, column_1 = 1;
    int row_2 = 7, column_2 = 3;
    // --------------------------------------------

    int rows = matrix.size();
    int cols = matrix[0].size();

    // 1) Initialize DSU over all cells
    Dsu dsu(rows * cols);

    // 2) One pass: union every cell with its equal‐valued neighbors
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            int u = i * cols + j;
            for (int k = 0; k < 4; k++) {
                int ni = i + dr[k], nj = j + dc[k];
                if (ni >= 0 && ni < rows && nj >= 0 && nj < cols
                    && matrix[ni][nj] == matrix[i][j]) {
                    int v = ni * cols + nj;
                    dsu.unite(u, v);
                    }
            }
        }
    }

    int a = (row_1 - 1) * cols + (column_1 - 1);
    int b = (row_2 - 1) * cols + (column_2 - 1);

    if (dsu.find(a) == dsu.find(b)) {
        cout << (matrix[row_1 - 1][column_1 - 1] == '0'
                    ? "binary\n"
                    : "decimal\n");
    } else {
        cout << "neither\n";
    }

    return 0;

}
