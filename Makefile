TARGET := target/release/simple-ringbuf
PERF_STAT_OPT:=-B -e cache-references,cache-misses,cycles,instructions,branch-misses,inst_retired,l1d_cache,l1d_cache_lmiss_rd,l2d_cache,l2d_cache_lmiss_rd,l3d_cache,l3d_cache_lmiss_rd,mem_access,stalled-cycles-frontend,stalled-cycles-backend

ARCH=$(shell uname -m)
ifeq ($(ARCH),aarch64)
	# for Arm Cortex-A78AE(Jetson Orin Series)
	# export RUSTFLAGS=-C target-feature=+v8.2a,+a78,+rcpc,+dotprod,+ssbs
endif

PROFILES:=opt-2 opt-s opt-z disable-lto release

.PHONY: build
build: ${TARGET}

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

.PHONY: bench.loop
bench.loop:
	@for i in ${PROFILES}; do\
		cargo build --profile $$i;\
		echo build $$i;\
		make bench TARGET=target/$$i/simple-ringbuf > bench_$$i.txt;\
	done
	sh make_csv.sh

.PHONY: bench
bench: ${TARGET}
	@echo ${TARGET}
	@${TARGET} -r r0s
	@${TARGET} -r r1s
	@${TARGET} -r r2s
	@${TARGET} -r r2m -c 0,0
	@${TARGET} -r r2m -c 0,1
	@${TARGET} -r r2m -c 0,4
	@${TARGET} -r r3s
	@${TARGET} -r r3m -c 0,0
	@${TARGET} -r r3m -c 0,1
	@${TARGET} -r r3m -c 0,4

.PHONY: perf.s
perf.s: ${TARGET}
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r2s
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r3s

.PHONY: perf.r2
perf.r2: ${TARGET}
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r1s
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r2s
	perf stat ${PERF_STAT_OPT} ${TARGET} -r r2m -c 0,1
#	perf stat ${PERF_STAT_OPT} ${TARGET} -r r2m -c 0,4

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
	cargo asm "<ringbuf::r1::Buffer as ringbuf::helper::RingBufTrait<usize>>::enqueue"
	cargo asm "<ringbuf::r1::Buffer as ringbuf::helper::RingBufTrait<usize>>::dequeue"
	cargo asm "<ringbuf::r2::Buffer as ringbuf::helper::RingBufTrait<usize>>::enqueue"
	cargo asm "<ringbuf::r2::Buffer as ringbuf::helper::RingBufTrait<usize>>::dequeue"
	cargo asm "<ringbuf::r3::Buffer as ringbuf::helper::RingBufTrait<usize>>::enqueue"
	cargo asm "<ringbuf::r3::Buffer as ringbuf::helper::RingBufTrait<usize>>::dequeue"

.PHONY: cpuinfo
cpuinfo:
	cat /proc/cpuinfo | grep -e processor -e "core id"
