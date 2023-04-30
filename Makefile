.DEFAULT_GOAL := help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
	| sort \
	| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

docker-back: ## enter the backend container
	docker exec -ti backend /bin/bash

docker-front: ## enter the frontend container
	docker exec -ti frontend /bin/bash

docker-blockchain-client: ## enter the blockchain-client container
	docker exec -ti blockchain-client /bin/bash

docker-blockchain: ## enter the blockchain container
	docker exec -ti blockchain /bin/bash

docker-bd: ## enter the mariadb container
	docker exec -ti mariadb /bin/bash

dev: ## run the backend in a dev environment (run inside the container)
	cargo run -p backend

backend-setup:
	docker exec -ti backend make dev

deploy: ## deploy the backend (run inside the container)
	cargo build --release -p backend
	cargo run -p backend

frontend-run: ## run the frontend (run inside the container)
	cd frontend && trunk serve --address 0.0.0.0 --port 3000

frontend-build:
	cd frontend && trunk build --release

frontend-deploy: frontend-build frontend-run

# db-setup: ## setup the db with diesel (run inside the container)
# 	diesel setup

db-setup:
	docker exec -ti backend diesel database setup

db-reset: ## reset the db (run inside the container)
	diesel database reset

db-migration:
	diesel migration run

db-redo:
	diesel migration redo

docker-dev: ## run docker and attach to a shell in the backend container
	docker-compose run --service-ports backend

docker-down: ## kill docker containers
	# docker-compose down --remove-orphans
	docker-compose down

setup: db-setup backend-setup
