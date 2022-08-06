EXEC_NAME = "shorten-path"
INSTALL_DIR = "$(HOME)/.local/bin"

## build: the executable
build:
	cargo build --release

.PHONY: install
## install: install the executable to the $HOME/.local/bin/ directory
install: build
	mkdir -p "$(INSTALL_DIR)"
	cp "target/release/$(EXEC_NAME)" "$(INSTALL_DIR)/$(EXEC_NAME)"


.PHONY: clean
clean:
	rm -rf target/*