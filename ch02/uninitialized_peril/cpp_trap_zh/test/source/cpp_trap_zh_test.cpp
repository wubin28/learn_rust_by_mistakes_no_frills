#include "lib.hpp"

auto main() -> int
{
  auto const lib = library {};

  return lib.name == "cpp_trap_zh" ? 0 : 1;
}
