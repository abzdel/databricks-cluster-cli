install-azure:
	curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash

compile:
	cd cluster-rs && cargo build --release
	cp cluster-rs/target/release/cluster .

