.PHONY: dev build css watch install

# Install dependencies
install:
	npm install
	cargo build

# Build Tailwind CSS once
css:
	./tailwindcss -i ./assets/input.css -o ./assets/output.css

# Build Tailwind CSS for production (minified)
css-prod:
	./tailwindcss -i ./assets/input.css -o ./assets/output.css --minify

# Build the entire project
build: css-prod
	cargo build --release

# Development mode: Run Tailwind watcher and Cargo concurrently
dev:
	@echo "Starting development server..."
	@trap 'kill %1; kill %2' SIGINT; \
	./tailwindcss -i ./assets/input.css -o ./assets/output.css --watch & \
	cargo run --bin rust-blog

# Watch only CSS
watch-css:
	./tailwindcss -i ./assets/input.css -o ./assets/output.css --watch
