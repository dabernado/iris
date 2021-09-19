/*
 * parse.h - IRIS parser
 *
 */

struct LT {
  char label[256];
  unsigned int address;
  struct LT *next;
};
