//
// Created by dj on 3/17/25.
//

#ifndef GRAPH_HPP
#define GRAPH_HPP

#include <vector>
#include <iostream>

using namespace std;

class Graph {

public:
    class Node {
    public:
        int m_row;
        int m_column;
        char m_value;
        vector<Node*> neighbors;

        Node(int row, int column, char value);

        void addNeighbors(vector<Node*> nodes);
    };

    int m_rows;
    int m_columns;
    vector<Node*> m_nodes;

    Graph(const vector<string>& matrix);

    ~Graph();
};

#endif // GRAPH_HPP
