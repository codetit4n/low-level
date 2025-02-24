cmake_minimum_required(VERSION 3.31)
project(allocator C)

set(CMAKE_C_STANDARD 99)

# Add include directory
include_directories(include)

# Define the static library
add_library(allocator STATIC src/allocator.c)

# Ensure the tests directory exists in the build
set(TESTS_DIR ${CMAKE_BINARY_DIR}/tests)
file(MAKE_DIRECTORY ${TESTS_DIR})

# Define the test executable **before linking**
add_executable(test_alloc tests/test_alloc.c)

# Now link the allocator library to test_alloc
target_link_libraries(test_alloc PRIVATE allocator)

# Move test_alloc into the tests directory
set_target_properties(test_alloc PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${TESTS_DIR})

# Enable testing and register test_alloc in CTest
enable_testing()
add_test(NAME AllocatorTest COMMAND ${TESTS_DIR}/test_alloc)
set_tests_properties(AllocatorTest PROPERTIES
    PASS_REGULAR_EXPRESSION "TEST PASSED"
    FAIL_REGULAR_EXPRESSION "TEST FAILED"
    TIMEOUT 10
    OUTPUT_ON_FAILURE ON  # <-- Forces output to be shown in `make test`
)
