dev:
	docker-compose -f docker/docker-compose.yml up -d

clean:
	docker-compose -f docker/docker-compose.yml down

watch-logs:
	docker logs -f local_postgres

open-postgres-console:
	docker exec -it local_postgres psql -U postgres

create-db:
	docker exec -it local_postgres psql -U postgres -c "create database wine_server"

add-current-user:
	docker exec -it local_postgres psql -U postgres -c "CREATE ROLE ${USER} WITH SUPERUSER LOGIN;"

start-server-dev:
	cargo run --bin start_server