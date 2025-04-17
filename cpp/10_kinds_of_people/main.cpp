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
    int rows, cols;
    cin >> rows >> cols;

    vector<string> matrix(rows);
    for (int i = 0; i < rows; i++) {
        cin >> matrix[i];
    }

    Dsu dsu(rows * cols);
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

    int numOfQueries;
    cin >> numOfQueries;
    while (numOfQueries--) {
        int row_1, col_1, row_2, col_2;
        cin >> row_1 >> col_1 >> row_2 >> col_2;
        int a = (row_1 - 1) * cols + (col_1 - 1);
        int b = (row_2 - 1) * cols + (col_2 - 1);
        if (dsu.find(a) == dsu.find(b)) {
            cout << (matrix[row_1 - 1][col_1 - 1] == '0' ? "binary\n" : "decimal\n");
        } else {
            cout << "neither\n";
        }
    }

    return 0;

    // vector<string> matrix = {
    //     "11111111111111111111",
    //     "11000000000000000101",
    //     "11111111111111110000",
    //     "11111111111111110000",
    //     "11000000000000000111",
    //     "00011111111111111111",
    //     "00111111111111111111",
    //     "10000000000000001111",
    //     "11111111111111111111",
    //     "11111111111111111111"
    // };
    //
    // int row_1 = 8, column_1 = 1;
    // int row_2 = 7, column_2 = 3;
    // // --------------------------------------------
    //
    // int rows = matrix.size();
    // int cols = matrix[0].size();
    //
    // Dsu dsu(rows * cols);
    //
    // for (int i = 0; i < rows; i++) {
    //     for (int j = 0; j < cols; j++) {
    //         int u = i * cols + j;
    //         for (int k = 0; k < 4; k++) {
    //             int ni = i + dr[k], nj = j + dc[k];
    //             if (ni >= 0 && ni < rows && nj >= 0 && nj < cols
    //                 && matrix[ni][nj] == matrix[i][j]) {
    //                 int v = ni * cols + nj;
    //                 dsu.unite(u, v);
    //                 }
    //         }
    //     }
    // }
    //
    // int a = (row_1 - 1) * cols + (column_1 - 1);
    // int b = (row_2 - 1) * cols + (column_2 - 1);
    //
    // if (dsu.find(a) == dsu.find(b)) {
    //     cout << (matrix[row_1 - 1][column_1 - 1] == '0'
    //                 ? "binary\n"
    //                 : "decimal\n");
    // } else {
    //     cout << "neither\n";
    // }
    //
    // return 0;

}
