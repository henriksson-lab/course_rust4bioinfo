.PHONY: all render preview clean help

all: render

render:
	quarto render

preview:
	quarto preview

# Remove the rendered site, Quarto's caches, and every exercise crate's
# `target/` directory (via `cargo clean`).
clean:
	rm -rf _site .quarto
	@find . -name Cargo.toml -not -path '*/target/*' -print0 | \
	  xargs -0 -n1 dirname | \
	  while read dir; do \
	    echo "cargo clean in $$dir"; \
	    (cd "$$dir" && cargo clean); \
	  done

help:
	@echo "Targets:"
	@echo "  make           render the whole site to _site/"
	@echo "  make preview   start a live-reload preview server"
	@echo "  make clean     remove _site/, .quarto/, and run cargo clean"
	@echo "                 in every exercise crate"

gitaddall:
	git add day*/*qmd
	git add day*/README.md
	git add day*/*/Cargo.toml
	git add day*/*/data/*
	git add day*/*/*/*rs

	git add day*/*/README.md
	git add day*/*/*/Cargo.toml
	git add day*/*/*/*/*rs
