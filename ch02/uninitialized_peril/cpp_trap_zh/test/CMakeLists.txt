# Parent project does not export its library target, so this CML implicitly
# depends on being added from it, i.e. the testing is done only from the build
# tree and is not feasible from an install location

project(cpp_trap_zhTests LANGUAGES CXX)

# ---- Tests ----

add_executable(cpp_trap_zh_test source/cpp_trap_zh_test.cpp)
target_link_libraries(cpp_trap_zh_test PRIVATE cpp_trap_zh_lib)
target_compile_features(cpp_trap_zh_test PRIVATE cxx_std_20)

add_test(NAME cpp_trap_zh_test COMMAND cpp_trap_zh_test)

# ---- End-of-file commands ----

add_folders(Test)
