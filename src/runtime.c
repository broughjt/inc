#include <stdio.h>

#define BOOLEAN_FALSE 0x2f
#define BOOLEAN_TRUE 0x6f

#define FIXNUM_MASK 0x03
#define FIXNUM_TAG 0x00
#define FIXNUM_SHIFT 2

#define CHARACTER_MASK 0x3f
#define CHARACTER_TAG 0x0f
#define CHARACTER_SHIFT 8

#define LIST_NULL 0x3f

typedef unsigned int ptr;

ptr scheme_entry();

static void print_char(char c)
{
  if      (c == '\t') printf("#\\tab");
  else if (c == '\n') printf("#\\newline");
  else if (c == '\r') printf("#\\return");
  else if (c == ' ')  printf("#\\space");
  else                printf("#\\%c", c);
}

static void print_ptr(ptr x)
{
  if ((x & FIXNUM_MASK) == FIXNUM_TAG)
    printf("%d", ((int) x) >> FIXNUM_SHIFT);
  else if (x == BOOLEAN_FALSE)
    printf("#f");
  else if (x == BOOLEAN_TRUE)
    printf("#t");
  else if (x == LIST_NULL)
    printf("()");
  else if ((x & CHARACTER_MASK) == CHARACTER_TAG)
    print_char(x >> CHARACTER_SHIFT);
  else
    printf("#<unknown 0x%08x>", x);
  printf("\n");
}

int main()
{
  print_ptr(scheme_entry());

  return 0;
}