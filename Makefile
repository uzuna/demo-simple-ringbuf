TARGET := target/release/simple-ringbuf
PERF_STAT_OPT:=-B -e cache-references,cache-misses,cycles,instructions,branch-misses,faults,migrations

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
	@${TARGET} -r r2m -c 0,0
	@${TARGET} -r r2m -c 0,1
	@${TARGET} -r r2m -c 0,2
	@${TARGET} -r r3s
	@${TARGET} -r r3m -c 0,0
	@${TARGET} -r r3m -c 0,1
	@${TARGET} -r r3m -c 0,2

.PHONY: perf.s
perf.s:
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r2s
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r3s

.PHONY: perf.r2
perf.r2:
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r2s
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r2m -c 0,0
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r2m -c 0,1

.PHONY: perf.r3
perf.r3:
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r3s
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r3m -c 0,0
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r3m -c 0,1
