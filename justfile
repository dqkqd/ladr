watch:
  mdbook serve --open

test:
  cargo test
  mdbook test

format:
  cargo fmt
  dprint fmt

typos:
  typos
