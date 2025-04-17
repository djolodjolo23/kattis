//
// Created by dj on 3/19/25.
//

#ifndef DFS_HPP
#define DFS_HPP

#include "Graph.hpp"
#include <vector>
#include <iostream>

class Dfs {
public:
    static void dfs(Graph::Node* start, const Graph::Node* end, int numOfRows, int numOfCols);
};

#endif //DFS_HPP