SILENT:
PHONY:
MIGRATION_NAME ?= new_migration

DB_CONN_DEV = "host=localhost user=postgres password=postgres port=5443 dbname=shortener sslmode=disable"

FOLDER_PG= migrations/pg

compose:
	docker-compose up -d

metrics:
	cargo watch -x 'run -p metrics-server'

url	:
	cargo watch -x 'run -p url-shortener'

run-prod:
	cargo run --release -p metrics

workspace:
	cargo watch -x 'run -p metrics-server' && cargo watch -x 'run -p url-shortener'

migrations-click-up:
	goose -dir migrations clickhouse "tcp://localhost:9000?username=default&password=clickhouse" up

migrations-up:
	goose -dir $(FOLDER_PG) postgres $(DB_CONN_DEV)   up

migrations-down:
	goose -dir $(FOLDER_PG) postgres $(DB_CONN_DEV)   down


migrations-status:
	goose -dir $(FOLDER_PG) postgres $(DB_CONN_DEV)  status

migrations-new:
	goose -dir $(FOLDER_PG) create $(MIGRATION_NAME) sql

docker:
	docker compose up -d

compose-prod:
	docker compose -f docker-compose.prod.yaml up --build -d
