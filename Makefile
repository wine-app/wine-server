dev:
	docker-compose -f docker/docker-compose.yml up -d

clean:
	docker-compose -f docker/docker-compose.yml down

watch-logs:
	docker logs -f local_postgres

open-postgres-console:
	docker exec -it local_postgres psql -U postgres

create-db:
	docker exec -it local_postgres psql -U postgres -c "create database diesel_demo"

add-current-user:
	docker exec -it local_postgres psql -U postgres -c "CREATE ROLE ${USER} WITH SUPERUSER LOGIN;"