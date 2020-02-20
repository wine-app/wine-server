up:
	docker-compose up -d

down:
	docker-compose down

ssh:
	docker exec -it wine_rust bash

dev:
	cargo run --bin start_server