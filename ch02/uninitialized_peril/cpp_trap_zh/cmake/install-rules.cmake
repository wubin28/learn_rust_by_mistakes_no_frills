install(
    TARGETS cpp_trap_zh_exe
    RUNTIME COMPONENT cpp_trap_zh_Runtime
)

if(PROJECT_IS_TOP_LEVEL)
  include(CPack)
endif()
