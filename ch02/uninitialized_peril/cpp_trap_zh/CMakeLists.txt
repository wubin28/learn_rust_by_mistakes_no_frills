cmake_minimum_required(VERSION 3.14)

include(cmake/prelude.cmake)

project(
    cpp_trap_zh
    VERSION 0.1.0
    DESCRIPTION "Short description"
    HOMEPAGE_URL "https://example.com/"
    LANGUAGES CXX
)

include(cmake/project-is-top-level.cmake)
include(cmake/variables.cmake)

# ---- Declare library ----

add_library(
    cpp_trap_zh_lib OBJECT
    source/lib.cpp
)

target_include_directories(
    cpp_trap_zh_lib ${warning_guard}
    PUBLIC
    "\$<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/source>"
)

target_compile_features(cpp_trap_zh_lib PUBLIC cxx_std_20)

# ---- Declare executable ----

add_executable(cpp_trap_zh_exe source/main.cpp)
add_executable(cpp_trap_zh::exe ALIAS cpp_trap_zh_exe)

set_property(TARGET cpp_trap_zh_exe PROPERTY OUTPUT_NAME cpp_trap_zh)

target_compile_features(cpp_trap_zh_exe PRIVATE cxx_std_20)

target_link_libraries(cpp_trap_zh_exe PRIVATE cpp_trap_zh_lib)

# ---- Install rules ----

if(NOT CMAKE_SKIP_INSTALL_RULES)
  include(cmake/install-rules.cmake)
endif()

# ---- Developer mode ----

if(NOT cpp_trap_zh_DEVELOPER_MODE)
  return()
elseif(NOT PROJECT_IS_TOP_LEVEL)
  message(
      AUTHOR_WARNING
      "Developer mode is intended for developers of cpp_trap_zh"
  )
endif()

include(cmake/dev-mode.cmake)
