//
// Created by dj on 3/17/25.
//

#include "Graph.hpp"


Graph::Node::Node(int row, int column, char value)
    : m_row(row), m_column(column), m_value(value) {}


Graph::Graph(const vector<string>& matrix) {
    m_rows = matrix.size();
    m_columns = matrix[0].size();
    m_nodes.resize(m_rows * m_columns, nullptr);

    for (int i = 0; i < m_rows; i++) {
        for (int j = 0; j < m_columns; j++) {
            const int index = i * m_columns + j;
            m_nodes[index] = new Node(i, j, matrix[i][j]);
        }
    }

    for (Node* node : m_nodes) {
        if (node->m_row != 0) {
            // above
            if (m_nodes[(node->m_row - 1) * m_columns + node->m_column]->m_value == node->m_value) {
                node->neighbors.push_back(m_nodes[(node->m_row - 1) * m_columns + node->m_column]);
            }
        }
        if (node->m_row != m_rows - 1) {
            // bellow
            if (m_nodes[(node->m_row + 1) * m_columns + node->m_column]->m_value == node->m_value) {
                node->neighbors.push_back(m_nodes[(node->m_row + 1) * m_columns + node->m_column]);
            }
        }
        if (node->m_column != 0) {
            // left
            if (m_nodes[node->m_row * m_columns + node->m_column - 1]->m_value == node->m_value) {
                node->neighbors.push_back(m_nodes[node->m_row * m_columns + node->m_column - 1]);
            }
        }
        if (node->m_column != m_columns - 1) {
            // right
            if (m_nodes[node->m_row * m_columns + node->m_column + 1]->m_value == node->m_value) {
                node->neighbors.push_back(m_nodes[node->m_row * m_columns + node->m_column + 1]);
            }
        }
    }
}

Graph::~Graph() {
    for (Node* node : m_nodes) {
        delete node;
    }
}