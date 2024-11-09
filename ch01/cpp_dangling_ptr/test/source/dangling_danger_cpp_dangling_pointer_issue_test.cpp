#include "lib.hpp"

auto main() -> int
{
  auto const lib = library {};

  return lib.name == "dangling_danger_cpp_dangling_pointer_issue" ? 0 : 1;
}
