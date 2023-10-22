format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run

install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

test:
	python -m pytest -vv --cov=main --cov=library test_*.py

format:	
	black *.py 

lint:
	pylint --disable=R,C --ignore-patterns=test_.*?py *.py library/*.py
	
all: install lint test format