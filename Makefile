TARGET = target/release/simple-ringbuf

.PHONY: fmt
fmt:
	cargo fmt
	git add -u
	cargo clippy --fix --allow-staged

.PHONY: check-fmt
check-fmt:
	cargo fmt --check
	cargo clippy

${TARGET}: src/*.rs
	cargo build --release

.PHONY: bench
bench:
	@${TARGET} -r r0
	@${TARGET} -r r1
	@${TARGET} -r r2s
	@${TARGET} -r r2m -s
	@${TARGET} -r r2m
	@${TARGET} -r r3s
	@${TARGET} -r r3m -s
	@${TARGET} -r r3m
