#include "helper.h"
#include "contestant.h"
#include <stdio.h>
#include <stdlib.h>

Group *find_or_create_group(Group **groups, int *group_count, int id) {
  for (int i = 0; i < *group_count; i++) {
    if ((*groups)[i].id == id)
      return &(((*groups)[i]));
  }

  *groups = realloc(*groups, sizeof(Group) * (*group_count + 1));
  Group *new_group = &((*groups)[*group_count]);
  new_group->id = id;
  new_group->entries = NULL;
  new_group->size = 0;
  new_group->capacity = 0;
  (*group_count)++;
  return new_group;
}

void add_contestant(Group *group, int v1, int v2, int v3) {
  if (group->size == group->capacity) {
    group->capacity = (group->capacity == 0) ? 4 : group->capacity * 2;
    group->entries =
        realloc(group->entries, sizeof(Contestant) * group->capacity);
  }
  group->entries[group->size++] = (Contestant){v1, v2, v3};
}
