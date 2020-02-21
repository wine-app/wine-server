start:
	docker-compose up

rebuild:
	docker-compose build --no-cache && docker-compose up