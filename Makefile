# Run the API locally with hot reloading.
run-dev:
	cargo lambda watch

# Send a request to the API locally.
request:
	curl -X GET http://localhost:9000/?from_ms=1706577426000&n_results=100

# Format the code with rustfmt.
format:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --quiet

# Lint the code with clippy.
lint:
	@rustup component add clippy 2> /dev/null
	@cargo-clippy --all-targets --all-features -- -D warnings 

# Run units tests to make sure you didn't break anything.
test:
	cargo test

# Build the binary for AWS Lambda.
build:
	cargo lambda build --release

# Deploy the binary to AWS Lambda.
deploy:
	cargo lambda deploy lambda-rust-api \
		--timeout 10 \
		--memory-size 1024 \
		--env-var RUST_LOG=info \
		--profile default # Use the one in ~/.aws/credentials

# Run all the commands above.
all: format lint test build deploy

# Send a request to the API deployed to AWS Lambda.
invoke:
	cargo lambda invoke lambda-rust-api \
		--data-file ./sample_request.json \
		--remote
		