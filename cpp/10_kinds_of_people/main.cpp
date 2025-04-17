#include <iostream>
#include <vector>
#include "Graph.hpp"
#include "Bfs.hpp"
#include "Dfs.hpp"
using namespace std;

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

    int rows = 1;
    int cols = 2;
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

    Graph graph(matrix);
    int row_1 = 8;
    int column_1 = 1;
    int row_2 = 7;
    int column_2 = 3;

    Graph::Node* start = graph.getNode(row_1 - 1, column_1 - 1);
    Graph::Node* end = graph.getNode(row_2 - 1, column_2 - 1);

    Bfs::bfs(start, end, rows, cols);
}
