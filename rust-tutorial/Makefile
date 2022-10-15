SCRIPT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

.PHONY: help
.DEFAULT_GOAL=help
help:  ## help for this Makefile
	@grep -E '^[a-zA-Z0-9_\-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: primitives
primitives:  ## primitives example
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo run --manifest-path=$@/Cargo.toml

strings:  ## strings example
.PHONY: strings
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo run --manifest-path=$@/Cargo.toml

.PHONY: functions
functions:  ## functions example
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo run --manifest-path=$@/Cargo.toml

.PHONY: conditional
conditional:  ## conditional statements example
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo run --manifest-path=$@/Cargo.toml

.PHONY: tuples
tuples:  ## tuples example
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo run --manifest-path=$@/Cargo.toml

.PHONY: structs
structs:  ## tuples example
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo run --manifest-path=$@/Cargo.toml

.PHONY: enums
enums:  ## enums example
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo run --manifest-path=$@/Cargo.toml

.PHONY: generics
generics:  ## generics example
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo run --manifest-path=$@/Cargo.toml

.PHONY: ownership
ownership:  ## ownership example
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo run --manifest-path=$@/Cargo.toml

.PHONY: lifetimes
lifetimes:  ## lifetimes example
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo run --manifest-path=$@/Cargo.toml

.PHONY: casting
casting:  ## casting, shadowing, constats, static example
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo run --manifest-path=$@/Cargo.toml

.PHONY: print-format
print-format:  ## casting, shadowing, constats, static example
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo run --manifest-path=$@/Cargo.toml

.PHONY: main-lib
main-lib:  ## casting, shadowing, constats, static example
	watchexec -w $@/src/main.rs --force-poll 500 -- cargo run --manifest-path=$@/Cargo.toml
