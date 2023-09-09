INSTALL_DIR=~/.local/bin
TARGET_DIR = ./target/release

all:
	cargo build --release

clean:
	cargo clean

install: all
	mkdir -p $(INSTALL_DIR)
	cp -f $(TARGET_DIR)/command-cache $(INSTALL_DIR)
