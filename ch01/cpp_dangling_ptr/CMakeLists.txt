cmake_minimum_required(VERSION 3.14)

include(cmake/prelude.cmake)

project(
    dangling_danger_cpp_dangling_pointer_issue
    VERSION 0.1.0
    DESCRIPTION "Short description"
    HOMEPAGE_URL "https://example.com/"
    LANGUAGES CXX
)

include(cmake/project-is-top-level.cmake)
include(cmake/variables.cmake)

# ---- Declare library ----

add_library(
    dangling_danger_cpp_dangling_pointer_issue_lib OBJECT
    source/lib.cpp
)

target_include_directories(
    dangling_danger_cpp_dangling_pointer_issue_lib ${warning_guard}
    PUBLIC
    "\$<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/source>"
)

target_compile_features(dangling_danger_cpp_dangling_pointer_issue_lib PUBLIC cxx_std_20)

# ---- Declare executable ----

add_executable(dangling_danger_cpp_dangling_pointer_issue_exe source/main.cpp)
add_executable(dangling_danger_cpp_dangling_pointer_issue::exe ALIAS dangling_danger_cpp_dangling_pointer_issue_exe)

set_property(TARGET dangling_danger_cpp_dangling_pointer_issue_exe PROPERTY OUTPUT_NAME dangling_danger_cpp_dangling_pointer_issue)

target_compile_features(dangling_danger_cpp_dangling_pointer_issue_exe PRIVATE cxx_std_20)

target_link_libraries(dangling_danger_cpp_dangling_pointer_issue_exe PRIVATE dangling_danger_cpp_dangling_pointer_issue_lib)

# ---- Install rules ----

if(NOT CMAKE_SKIP_INSTALL_RULES)
  include(cmake/install-rules.cmake)
endif()

# ---- Developer mode ----

if(NOT dangling_danger_cpp_dangling_pointer_issue_DEVELOPER_MODE)
  return()
elseif(NOT PROJECT_IS_TOP_LEVEL)
  message(
      AUTHOR_WARNING
      "Developer mode is intended for developers of dangling_danger_cpp_dangling_pointer_issue"
  )
endif()

include(cmake/dev-mode.cmake)
