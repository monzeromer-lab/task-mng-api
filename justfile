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
    env RUST_LOG=info SECRET_KEY=23k4n5lj3n4l5nj4oin23on534n5jn345oi34n5k324n5okn342o5jn RUST_BACKTRACE=full cargo run {{args}}
