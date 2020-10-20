# Makefile
# help:
#    all)
#    make --always-make
.DEFAULT_GOAL := help
MAKEFILE_DIR := $(dir $(lastword $(MAKEFILE_LIST)))

.PHONY: help
help:
	@echo "\n>> help [ command list ]"
	@grep -E '^.PHONY: [a-zA-Z_-]+.*?##' $(MAKEFILE_LIST) | awk 'BEGIN {FS = " "}; {printf "\033[35m%-30s\033[32m %s\n", $$2, $$4}'
	@echo ""

.PHONY: gitignore ## [category]`description`.
gitignore:
	gibo dump windows macos linux rust 

.PHONY: setup ## [category]`description`.
setup:
	cargo new boilerplate-rust
	
.PHONY: build ## [category]`description`.
build:
	cargo build
