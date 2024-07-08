# Default recipe which runs `just build-release`
default: run

# Compiles with debug profile
build-debug *args:
    cargo build {{args}}

# Compiles with release profile
build-release *args: (build-debug '--release' args)

# Runs a clippy check
check *args:
    cargo clippy --all-features {{args}} -- -W clippy::pedantic

# Runs a clippy check with JSON message format
check-json: (check '--message-format=json')

# Profile memory usage with heaptrack
heaptrack:
    cargo heaptrack --profile release-with-debug

dev *args:
    cargo fmt
    just run {{args}}

# Run with debug logs
run *args:
    env RUST_LOG=info RUST_BACKTRACE=full cargo run {{args}}
