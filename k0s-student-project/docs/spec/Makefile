.DEFAULT_GOAL := help

MD_SOURCES := $(shell find . -name '*.md' -not -name 'dist.md' | sort)

.PHONY: build serve
serve: dist.md ## run development serve with live reloading
	livedown start dist.md --open
	# TODO: Watch for changes and rebuild dist.md
build: dist.md ## create a .pdf with all the docs, nicely formatted
	# TODO: Implement this
	echo "Not implemented"
	false

dist.md: $(MD_SOURCES)
	cat $(MD_SOURCES) > dist.md

.PHONY: help
help: ## print this message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
