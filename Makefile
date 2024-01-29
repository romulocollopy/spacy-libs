dev:
	sleep infinity

test:
	cargo watch -i **/target -i db-data -x 'nextest run'
