#include <stdio.h>
#include <stdlib.h>

int main(int argc, [[maybe_unused]] char *argv[argc + 1]) { 
  // We can also remove [[maybe_unused]] for char* argv, remember [[features]] are a new add on C23
  // Declarations
  // Here we are using a designated initializer
  // Empty indexes like [1] and [3] will be initialized to 0.0 by default
  // (zero-initialization)
  double A[5] = {
    [0] = 12.8, // See the firts position of a Array is 0.
    [2] = 0.0555,
    [4] = 2.88,
  };
  // Loop: (declaration and initialization, condition, increment){body loop}
  for (size_t i = 0; i < 5; ++i) {
    // To use printf we need to include stdio.h header, which already has printf
    // defined
    printf(
      "Current interation is %zu which number is %g and it's square is %g \n",
      i, A[i], A[i] * A[i]);
    };
    return EXIT_SUCCESS;
  }
