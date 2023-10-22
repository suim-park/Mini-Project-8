install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

test:
	python -m pytest -vv --cov=main --cov=library test_*.py

format:	
	black *.py
	black library/*.py 

lint:
	pylint --disable=R,C --ignore-patterns=test_.*?py *.py library/*.py
	
all: install lint test format

activate:
	source /home/vscode/venv/bin/activate