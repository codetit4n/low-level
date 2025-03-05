# Variables
BUILD_DIR = build
CMAKE = cmake
MAKE = make
CMAKE_FLAGS = -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

.PHONY: all clean rebuild test

# Default build target
all:
	@mkdir -p $(BUILD_DIR)
	@cd $(BUILD_DIR) && $(CMAKE) .. $(CMAKE_FLAGS)
	@$(MAKE) -C $(BUILD_DIR)

# Clean the build directory
clean:
	@rm -rf $(BUILD_DIR)

# Rebuild everything (clean + build)
rebuild: clean all

# Run tests
test:
	@cd $(BUILD_DIR) && ctest --output-on-failure

