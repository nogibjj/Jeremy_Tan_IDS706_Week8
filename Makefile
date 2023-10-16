rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable 

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release

all: format lint test run

python_install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

python_test:
	python -m pytest -vv --cov=main --cov=mylib test_*.py

python_format:	
	black *.py 

python_lint:
	ruff check *.py mylib/*.py

python_container-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

python_refactor: format lint

python_deploy:
	#deploy goes here
		
python_all: install lint test format deploy

generate_and_push:
	# Add, commit, and push the generated files to GitHub
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add metric log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi