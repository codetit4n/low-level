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
add_executable(test_ealloc tests/test_ealloc.c)

# Now link the allocator library to test_ealloc
target_link_libraries(test_ealloc PRIVATE allocator)

# Move test_ealloc into the tests directory
set_target_properties(test_ealloc PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${TESTS_DIR})

# Enable testing and register test_ealloc in CTest
enable_testing()
add_test(NAME AllocatorTest COMMAND test_ealloc) # No regex needed; CTest checks exit status
