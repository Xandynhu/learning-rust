all: build-debug

build-debug:
	@cargo build
	@rm -rf bin && mkdir bin
	@cp target/debug/hello_cargo bin/hello_cargo
	@cp target/debug/guessing_game bin/guessing_game
	@cp target/debug/method_syntax bin/method_syntax