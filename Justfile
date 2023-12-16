watch EXAMPLE:
    cargo watch -x 'run --example=day{{EXAMPLE}} --release'

run IDX:
    cargo run --example=day{{IDX}}

release IDX:
    cargo run --example=day{{IDX}} --release

profile IDX:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --example=day{{IDX}}

clippy IDX:
    cargo clippy --example=day{{IDX}}

test:
    cargo test --release
    cargo test --examples --release
