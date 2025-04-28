#ifndef CONTESTANT_H
#define CONTESTANT_H

typedef struct {
  int problem_solved;
  int time_penalty;
  int time_for_last_accepted_solution;
  int bonus;
} Contestant;

typedef struct {
  int id;
  Contestant *entries;
  int size;
  int capacity;
} Group;

#endif
