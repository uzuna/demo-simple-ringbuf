TARGET := target/release/simple-ringbuf
PERF_STAT_OPT:=-B -e cache-references,cache-misses,cycles,instructions,branch-misses,faults,migrations

# nativeではaesのfeatureだけなので追加する
# a78
# dotprod　=　a8.4aのSDOT and UDOT -> A78に含まれる
# ras = RAS extension -> A78に含まれる
# rcpc　=　a8.3aのLDAPR命令
# ssbs = a8.5a peculativeStore Bypass Safe
# sha2 = Cryptographic extension -> OptionalでJetsonには含まれる
export RUSTFLAGS=-C target-feature=+v8.2a,+a78,+rcpc,+sha2,+ssbs

.PHONY: fmt
fmt:
	cargo fmt
	git add -u
	cargo clippy --fix --allow-staged

.PHONY: check-fmt
check-fmt:
	cargo fmt --check
	cargo clippy

${TARGET}: ringbuf-app/src/*.rs
	cargo build --release

.PHONY: bench
bench: ${TARGET}
	@${TARGET} -r r0s
	@${TARGET} -r r1s
	@${TARGET} -r r2s
	@${TARGET} -r r2m -c 0,0
	@${TARGET} -r r2m -c 0,1
	@${TARGET} -r r2m -c 0,4
	@${TARGET} -r r3s
	@${TARGET} -r r3m -c 0,0
	@${TARGET} -r r3m -c 0,1
	@${TARGET} -r r3m -c 0,2
	@${TARGET} -r r3m -c 0,3
	@${TARGET} -r r3m -c 0,4

.PHONY: perf.s
perf.s: ${TARGET}
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r2s
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r3s

.PHONY: perf.r2
perf.r2: ${TARGET}
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r2s
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r2m -c 0,1

.PHONY: perf.r3
perf.r3: ${TARGET}
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r3s
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r3m -c 0,1

.PHONY: enable-perf
enable-perf:
	sudo bash -c "echo -1 > /proc/sys/kernel/perf_event_paranoid"

.PHONY: setup
setup:
	sudo apt install -y linux-tools-common linux-tools-generic linux-tools-`uname -r`
	cargo install cargo-asm

.PHONY: check-asm
check-asm: ${TARGET}
	cargo asm "<simple_ringbuf::r2::Producer<T> as simple_ringbuf::helper::RingBufProducer<T>>::enqueue"
	cargo asm "<simple_ringbuf::r3::Producer<T> as simple_ringbuf::helper::RingBufProducer<T>>::enqueue"

.PHONY: cpuinfo
cpuinfo:
	cat /proc/cpuinfo | grep -e processor -e "core id"
