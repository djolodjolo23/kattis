#ifndef HELPER_H
#define HELPER_H

#include "contestant.h"

Group* find_or_create_group(Group **groups, int *group_count, int id);
void add_contestant(Group *group, int v1, int v2, int v3);

#endif

