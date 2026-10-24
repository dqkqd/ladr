watch:
  mdbook serve

test:
  cargo test
  mdbook test

format:
  cargo fmt
  cargo fmt --manifest-path mdbook-hidden-include/Cargo.toml
  dprint fmt

format-check:
  cargo fmt --check
  cargo fmt --check --manifest-path mdbook-hidden-include/Cargo.toml
  dprint check

typos:
  typos

markdownlint:
  markdownlint-cli2

check: format-check typos markdownlint test
