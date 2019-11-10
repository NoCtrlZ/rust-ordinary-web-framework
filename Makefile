run: src/main.rs
	cargo run src/main.rs
get:
	curl localhost:5000
post:
	curl -X POST -H "Content-Type: application/json" -d '{"Name":"NoCtrlZ", "Age":"23"}' localhost:5000