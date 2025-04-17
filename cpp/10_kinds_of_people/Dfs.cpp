//
// Created by dj on 3/19/25.
//
#include "Dfs.hpp"
#include <functional>
#include "Graph.hpp"
#include <vector>
#include <iostream>

void Dfs::dfs(Graph::Node* start, const Graph::Node* end, int numOfRows, int numOfCols) {
    if (start == end) {
        std::cout << start->m_label << std::endl;
        return;
    }

    std::vector<bool> visited(numOfRows * numOfCols, false);

    std::function<bool(Graph::Node*)> dfsHelper = [&](Graph::Node* current) -> bool {
        if (current == end) {
            std::cout << current->m_label << std::endl;
            return true;
        }

        visited[current->m_row * numOfCols + current->m_column] = true;

        for (Graph::Node* neighbor : current->neighbors) {
            const int index = neighbor->m_row * numOfCols + neighbor->m_column;
            if (!visited[index]) {
                if (dfsHelper(neighbor)) {
                    return true;
                }
            }
        }
        return false;
    };

    if (!dfsHelper(start)) {
        std::cout << "neither" << std::endl;
    }
}