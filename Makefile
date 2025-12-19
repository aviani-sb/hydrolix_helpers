clean_code:
	cargo clippy
	cargo fmt
	cargo build

code-coverage:
	rm -f *.profraw
	cargo tarpaulin --out Html

test:
	cargo nextest run
	
push: 
	cargo publish 
	
help: 
	@echo "CLEAN CODE"
	@echo "=====> make clean_code"
	@echo
	@echo "UNITTESTS"
	@echo "=====> cargo nextest list"
	@echo "=====> You can run a single test like this:"
	@echo "=====> cargo nextest run --no-capture *TEST*"
	@echo
