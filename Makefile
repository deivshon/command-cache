DEST_DIR = ~/.local/scripts
TARGET_DIR = ./target/release

all:
	cargo build --release

clean:
	cargo clean

install: all
	mkdir -p $(DEST_DIR)
	cp -f $(TARGET_DIR)/command-cache $(DEST_DIR)
