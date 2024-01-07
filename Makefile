lint:
	cargo fmt -v --all

test:
	cargo test --no-fail-fast -r --frozen --locked --offline

test-fast:
	cargo test -- --nocapture

coverage-lcov:
	cargo llvm-cov --frozen --locked --offline --lcov --output-path ./target/coverage/report/report.lcov

coverage-json:
	cargo llvm-cov --frozen --locked --offline --json --output-path ./target/coverage/report/report.json

coverage-html:
	cargo llvm-cov --frozen --locked --offline --html --output-dir ./target/coverage/report/

coverage-text:
	cargo llvm-cov --frozen --locked --offline

coverage-clean:
	# Delete previous profiles
	rm -rf ./target/coverage/profile/*
	# Delete previous reports
	rm -rf ./target/coverage/report/*
	# Setup coverage structure
	mkdir -p ./target/coverage/report

coverage: coverage-clean coverage-lcov coverage-json coverage-html coverage-text

clean: coverage-clean
	cargo clean

docs:
	cargo doc --frozen --locked --offline --bins --no-deps
