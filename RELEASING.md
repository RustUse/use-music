# Releasing

use-music uses lockstep 0.x versioning for focused crates and the facade crate.

Run the local validation path before publishing:

``sh
cargo fmt
cargo clippy --workspace --all-targets --all-features
cargo test --workspace --all-features
``

For a first publish, publish focused crates before the facade crate so crates.io can resolve sibling package versions. The music interval crate publishes as use-music-interval while its Rust crate name remains use_interval.
