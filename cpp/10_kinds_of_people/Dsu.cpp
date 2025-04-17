//
// Created by Djordje Dimitrov on 2025-04-17.
//

#include "Dsu.h"
#include <numeric>

Dsu::Dsu(int N) : p(N), r(N, 0) {
    std::iota(p.begin(), p.end(), 0);
}

int Dsu::find(int x) {
    return (p[x] == x ? x : p[x] = find(p[x]));
}

void Dsu::unite(int a, int b) {
    a = find(a);
    b = find(b);
    if (a == b) return;
    if (r[a] < r[b]) std::swap(a, b);
    p[b] = a;
    if (r[a] == r[b]) ++r[a];
}

