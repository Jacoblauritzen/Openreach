# Openreach (Rust) — developer shortcuts.
BIN := ./target/release/openreach

.PHONY: build setup onboard run admin test reset fmt clippy check

## build: compile the release binary (add FEATURES=fastembed for real embeddings)
build:
	cargo build --release $(if $(FEATURES),--features $(FEATURES),)

## setup: create the database and bootstrap the CRM
setup: build
	$(BIN) migrate
	$(BIN) setup-crm
