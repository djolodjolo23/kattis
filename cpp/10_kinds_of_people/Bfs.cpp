// Bfs.cpp proper indexed version
#include "Bfs.hpp"
#include <vector>

void Bfs::bfs(Graph::Node* start, const Graph::Node* end, int numOfRows, int numOfCols) {
    if (start == end) {
        std::cout << start->m_label << std::endl;
        return;
    }


    std::vector<bool> visitedFromStart(numOfRows * numOfCols, false);
    std::vector<bool> visitedFromEnd(numOfRows * numOfCols, false);

    std::queue <Graph::Node*> qStart;
    std::queue <Graph::Node*> qEnd;

    qStart.push(start);
    qEnd.push(const_cast<Graph::Node*>(end));

    visitedFromStart[start->m_row * numOfCols + start->m_column] = true;
    visitedFromEnd[end->m_row * numOfCols + end->m_column] = true;

    while (!qStart.empty() && !qEnd.empty()) {
        int startSize = qStart.size();
        for (int i = 0; i < startSize; i++) {
            Graph::Node* current = qStart.front();
            qStart.pop();

            int index = current->m_row * numOfCols + current->m_column;
            if (visitedFromEnd[index]) {
                std::cout << current->m_label << std::endl;
                return;
            }

            for (Graph::Node* neighbor : current->neighbors) {
                int neighborIndex = neighbor->m_row * numOfCols + neighbor->m_column;
                if (!visitedFromStart[neighborIndex]) {
                    visitedFromStart[neighborIndex] = true;
                    qStart.push(neighbor);
                }
            }
        }
        int endSize = qEnd.size();
        for (int i = 0; i < endSize; i++) {
            Graph::Node* current = qEnd.front();
            qEnd.pop();

            int index = current->m_row * numOfCols + current->m_column;
            if (visitedFromStart[index]) {
                std::cout << current->m_label << std::endl;
                return;
            }

            for (Graph::Node* neighbor : current->neighbors) {
                int neighborIndex = neighbor->m_row * numOfCols + neighbor->m_column;
                if (!visitedFromEnd[neighborIndex]) {
                    visitedFromEnd[neighborIndex] = true;
                    qEnd.push(neighbor);
                }
            }
        }
    }
    std::cout << "neither" << std::endl;
}