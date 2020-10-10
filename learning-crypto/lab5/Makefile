.DEFAULT_GOAL := help

PROG := hash
RUST_DOCKER_VERSION := 1.39

PROG_RS := $(PROG)-rs
ARTIFACT_PATH := $(PROG_RS)/target
MANIFEST_FLAG := --manifest-path $(PROG_RS)/Cargo.toml

# Porcelain
# ###############
.PHONY: build test init package run

almost: hash.txt hash_.txt ## all the dependencies

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

hash.txt: hash.pdf personal.txt
	./compute-hashes $^ > $@

hash_.txt: hash.pdf personal_.txt
	./compute-hashes $^ > $@

hash.pdf:
	wget https://inf.ug.edu.pl/~amb/krypto/hash.pdf


# Utilities
# ###############
.PHONY: help
help: ## print this message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
