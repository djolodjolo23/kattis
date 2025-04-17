//
// Created by dj on 3/19/25.
//
#pragma once

#include "Graph.hpp"
#include <queue>
#include <unordered_set>
#include <iostream>

#ifndef BFS_HPP
#define BFS_HPP

class Bfs {
public:
    static void bfs(Graph::Node* start, const Graph::Node* end, int numOfRows, int numOfCols);
};

#endif //BFS_HPP