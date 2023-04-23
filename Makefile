dev:
	cargo run -p backend

deploy: backend-deploy-build
	cargo run -p backend

backend-deploy-build:
	cargo build --release -p backend

frontend-run:
	trunk serve --address 0.0.0.0 --dist ./target/dist ./frontend/index.html

frontend-build:
	trunk build --release --dist ./dist ./frontend/index.html

db-setup:
	diesel setup

db-migration:
	diesel migration run

db-redo:
	diesel migration redo

docker-dev:
	docker-compose run --service-ports backend

docker-deploy:
	docker-compose run --service-ports backend make deploy
