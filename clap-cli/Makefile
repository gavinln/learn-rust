SCRIPT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

.PHONY: help
.DEFAULT_GOAL=help
help:  ## help for this Makefile
	@grep -E '^[a-zA-Z0-9_\-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: package-hunter
package-hunter:  ## run package-hunger
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo build --manifest-path=$@/Cargo.toml

.PHONY: run
run:  ## run package-hunger with arguments
	# cargo run --manifest-path=package-hunter/Cargo.toml -- count src
	cargo run --manifest-path=package-hunter/Cargo.toml -- -v count src
