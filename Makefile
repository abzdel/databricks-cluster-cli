install:
	curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash
	pip install --upgrade pip &&\
		pip install -r requirements.txt

compile:
	cd cluster-rs && cargo build --release
	cp cluster-rs/target/release/cluster .

format:
	chmod +x format.sh
	@echo "Formatting all projects with cargo"
	./format.sh

lint:
	chmod +x lint.sh
	@echo "Linting all projects with cargo"
	@rustup component add clippy 2> /dev/null
	./lint.sh

test:
	chmod +x test.sh
	@echo "Testing all projects with cargo"
	./test.sh

all: compile format lint test