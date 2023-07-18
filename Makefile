# Dev

run:
	cargo run -p bot

dev:
	bacon dev

clippy:
	bacon clippy

# Build

build:
	docker build -t affax/clazza-bot .

release-docker:
	cargo build --release
	docker compose -f docker-compose.yml down
	docker compose -f docker-compose.yml up --build -d
