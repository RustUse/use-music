.PHONY: help fmt check lint test test-minimal build doc examples publish-dry-run-focused publish-dry-run-facade release-readiness facade-post-publish-validation verify

FOCUSED_CRATES := use-note use-pitch use-music-interval use-scale use-mode use-key use-chord use-rhythm use-meter use-tempo use-dynamics use-articulation use-notation use-midi use-tuning
FACADE_CRATE := use-music

help:
	@printf "%s\n" \
		"fmt                            Check formatting with rustfmt" \
		"check                          Run cargo check for the workspace" \
		"lint                           Run clippy with warnings denied" \
		"test                           Run workspace tests with all features" \
		"test-minimal                   Run workspace tests with no default features" \
		"build                          Build the workspace with all features" \
		"doc                            Build workspace docs without dependencies" \
		"examples                       Check all examples" \
		"publish-dry-run-focused        Dry-run publish focused crates" \
		"publish-dry-run-facade         Dry-run publish facade after propagation" \
		"verify                         Run the main workspace validation path"

fmt:
	cargo fmt --all -- --check

check:
	cargo check --workspace --all-features

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

test-minimal:
	cargo test --workspace --no-default-features

build:
	cargo build --workspace --all-features

doc:
	cargo doc --workspace --all-features --no-deps

examples:
	cargo check --workspace --all-features --examples

publish-dry-run-focused:
	@if [ -z "$(strip $(FOCUSED_CRATES))" ]; then \
		printf "%s\n" "No focused crates configured"; \
	else \
		for crate in $(FOCUSED_CRATES); do \
			cargo package --list -p $$crate; \
			cargo publish --dry-run --allow-dirty -p $$crate; \
		done; \
	fi

publish-dry-run-facade:
	cargo publish --dry-run --allow-dirty -p $(FACADE_CRATE)

release-readiness: verify examples test-minimal publish-dry-run-focused

facade-post-publish-validation: publish-dry-run-facade

verify: fmt lint test build
