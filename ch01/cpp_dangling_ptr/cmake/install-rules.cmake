install(
    TARGETS dangling_danger_cpp_dangling_pointer_issue_exe
    RUNTIME COMPONENT dangling_danger_cpp_dangling_pointer_issue_Runtime
)

if(PROJECT_IS_TOP_LEVEL)
  include(CPack)
endif()
