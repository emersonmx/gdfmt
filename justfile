@build:
    cargo build

@run:
    cat ./samples/run.gd | cargo run --quiet --

@watch:
    bacon

@format:
    cargo fmt

@lint:
    cargo clippy

@lint-fix:
    cargo clippy --fix --allow-dirty

@ci:
    cargo fmt --check
    cargo clippy

@test:
    cargo test

@clean:
    cargo clean
