//
// Created by dj on 3/17/25.
//

#include "Graph.hpp"

#include <utility>


Graph::Node::Node(const int row, const int column, const char value, string label)
    : m_row(row), m_column(column), m_value(value), m_label(std::move(label)) {}


Graph::Graph(const vector<string>& matrix) {
    m_rows = matrix.size();
    m_columns = matrix[0].size();
    m_nodes.resize(m_rows * m_columns, nullptr);
    for (int i = 0; i < m_rows; i++) {
        for (int j = 0; j < m_columns; j++) {
            string label = (matrix[i][j] == '0') ? "binary" : "decimal";
            m_nodes[i * m_columns + j] = new Node(i, j, matrix[i][j], label);
        }
    }


    const int directionsX[] = {-1, 1, 0, 0};
    const int directionsY[] = {0, 0, -1, 1};

    for (int i = 0; i < m_rows; i++) {
        for (int j = 0; j < m_columns; j++) {
            const int currentIdx = i * m_columns + j;
            const char currentValue = m_nodes[currentIdx]->m_value; 

            for (int k = 0; k < 4; k++) {
                int newRow = i + directionsX[k];
                int newColumn = j + directionsY[k];
                if (newRow >= 0 && newRow < m_rows && newColumn >= 0 && newColumn < m_columns) {
                    const int neighborIdx = newRow * m_columns + newColumn;
                    if (m_nodes[neighborIdx]->m_value == currentValue) {
                        m_nodes[currentIdx]->neighbors.push_back(m_nodes[neighborIdx]);
                    }
                }
            }
        }
    }
}

Graph::Node* Graph::getNode(int row, int column) const {
    return m_nodes[row * m_columns + column];
}

Graph::~Graph() {
    for (Node* node : m_nodes) {
        delete node;
    }
}