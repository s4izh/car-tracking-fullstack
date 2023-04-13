back-dev:
	cargo run -p backend

back-build:
	cargo build --release -p backend

db-migration:
	diesel migration run

db-redo:
	diesel migration redo
