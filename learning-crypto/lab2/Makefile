.DEFAULT_GOAL := help
PROG := vigenere
PROG_RS := $(PROG)-rs

RUST_DOCKER_VERSION := 1.39
ARTIFACT_PATH := $(PROG_RS)/target


# Porcelain
# ###############
.PHONY: build test init

run:
	cargo run --manifest-path vigenere-rs/Cargo.toml -- $(ARGS)

build: ## create artifact
	cargo build --manifest-path vigenere-rs/Cargo.toml --release 

test: ## run all tests
	cargo test --manifest-path vigenere-rs/Cargo.toml --release 

init: ## one time setup
	direnv allow .

package: ## create production artifact
	docker run \
		--rm \
		--user "$$(id -u)":"$$(id -g)" \
		-v "$$PWD/$(PROG_RS)":/usr/src/myapp \
		-w /usr/src/myapp rust:$(RUST_DOCKER_VERSION)-slim \
		cargo build \
		--release

	rm -rf $(ARTIFACT_PATH)/debug
	find $(ARTIFACT_PATH)/release -mindepth 1 -prune -not -name '$(PROG_RS)' -print0 | xargs -0 rm -r

	tar -cvf $(PROG).tar .

# Plumbing
# ###############


# Utilities
# ###############
.PHONY: help
help: ## print this message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
