dev:
	sass styles/index.scss:static/styles.min.css --watch --no-source-map --style=compressed & cargo run

build:
	cargo build && sass styles/index.scss:static/styles.min.css --no-source-map --style=compressed
