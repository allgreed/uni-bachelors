.DEFAULT_GOAL := help

PROG := elgamal
RUST_DOCKER_VERSION := 1.39

PROG_RS := $(PROG)-rs
ARTIFACT_PATH := $(PROG_RS)/target
MANIFEST_FLAG := --manifest-path $(PROG_RS)/Cargo.toml

# Porcelain
# ###############
.PHONY: build test init package run

run:
	cargo run $(MANIFEST_FLAG)	-- $(ARGS)

build: ## create artifact
	cargo build $(MANIFEST_FLAG)

test: ## run all tests
	cargo test $(MANIFEST_FLAG)

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
