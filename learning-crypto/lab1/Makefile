.DEFAULT_GOAL := help


# Porcelain
# ###############
.PHONY: build test init

run:
	cargo run --manifest-path cezar-rs/Cargo.toml -- $(ARGS)

build: ## create artifact
	cargo build --manifest-path cezar-rs/Cargo.toml --release 

test: ## run all tests
	cargo test --manifest-path cezar-rs/Cargo.toml --release 

init: ## one time setup
	direnv allow .

# Plumbing
# ###############


# Utilities
# ###############
.PHONY: help
help: ## print this message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
