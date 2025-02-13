OUTPUT_DIR = ./c_output

OPENGL_WRAPPER_LIB_DIR = ./opengl_wrapper_lib
OPENGL_WRAPPER_LIB_SRC = $(OPENGL_WRAPPER_LIB_DIR)/opengl_wrapper_lib.c
OPENGL_WRAPPER_LIB_OBJ = $(OUTPUT_DIR)/opengl_wrapper_lib.o
OPENGL_WRAPPER_LIB = openglwrapper
OPENGL_WRAPPER_LIB_SO = $(OUTPUT_DIR)/lib$(OPENGL_WRAPPER_LIB).so

TEST_GAME_DIR = ./c_test_game
TEST_GAME_SRC = $(TEST_GAME_DIR)/test_game.c
TEST_GAME_EXE = $(OUTPUT_DIR)/test_game_exe

.PHONY: build-c
build-c:
	@echo "Building OpenGL Wrapper library..."
	gcc -c -fPIC $(OPENGL_WRAPPER_LIB_SRC) -o $(OPENGL_WRAPPER_LIB_OBJ) # Compile source file
	gcc -shared -o $(OPENGL_WRAPPER_LIB_SO) $(OPENGL_WRAPPER_LIB_OBJ) -lglfw -lGL # Create shared library

.PHONY: run-c
run-c: build-c
	@echo "Running Test Game..."
	gcc $(TEST_GAME_SRC) -o $(TEST_GAME_EXE) -L$(OUTPUT_DIR) -l$(OPENGL_WRAPPER_LIB) -Wl,-rpath,$(OUTPUT_DIR) # Compile source file
	$(TEST_GAME_EXE) # Run the test game

.PHONY: test-rust
test-rust: build-c
	@echo "Running Rust Tests Serially..."
	cargo test tests::test_simple_game_loop -- --nocapture
	cargo test tests::test_sprite_rendering -- --nocapture
	cargo test tests::test_screen_clearing -- --nocapture
	cargo test tests::test_key_presses -- --nocapture
	cargo test tests::test_sprite_position_update -- --nocapture
	
.PHONY: run-pingpong
run-pingpong: build-c
	@echo "Running Ping-Pong..."
	cargo run --bin pingpong
	
.PHONY: run-videogame
run-videogame: build-c
	@echo "Running videogame..."
	cargo run --bin videogame
	
clean :
	rm -rf *.o 
	rm $(objects) $(OPENGL_WRAPPER_LIB_SO) $(OPENGL_WRAPPER_LIB_OBJ)
	cargo clean