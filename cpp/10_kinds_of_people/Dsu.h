//
// Created by Djordje Dimitrov on 2025-04-17.
//

#ifndef DSU_H
#define DSU_H

#include <vector>

struct Dsu {

    Dsu(int N);
    int find (int x);
    void unite(int a, int b);

private:
    std::vector<int> p, r;

};


#endif //DSU_H
