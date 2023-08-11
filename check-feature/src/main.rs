#![recursion_limit = "512"]

// This script prints all cpu features which active in this build.
// There are 3 steps in usage of script:
// 1. get list of features using `rustc --print target-features`
// 2. put it into script (it has values actual for 2020-12-06 for x86-64 target).
// 3. run script.

fn pad_name(s: &str) -> String {
    let mut res = s.to_string();
    while res.len() < 30 {
        res.push(' ');
    }
    res
}

macro_rules! print_if_feature_enabled {
    () => {};
    ($feature:literal $(, $feats:literal)*)=>{
        if cfg!(target_feature = $feature){
            println!("feature {}", pad_name($feature));
        }
        print_if_feature_enabled!($($feats),*)
    }
}

fn main() {
    if cfg!(target_arch = "x86_64") {
        print_if_feature_enabled!(
            "adx",
            "aes",
            "avx",
            "avx2",
            "avx512bf16",
            "avx512bitalg",
            "avx512bw",
            "avx512cd",
            "avx512dq",
            "avx512er",
            "avx512f",
            "avx512ifma",
            "avx512pf",
            "avx512vbmi",
            "avx512vbmi2",
            "avx512vl",
            "avx512vnni",
            "avx512vp2intersect",
            "avx512vpopcntdq",
            "bmi1",
            "bmi2",
            "cmpxchg16b",
            "ermsb",
            "f16c",
            "fma",
            "fxsr",
            "gfni",
            "lzcnt",
            "movbe",
            "pclmulqdq",
            "popcnt",
            "rdrand",
            "rdseed",
            "rtm",
            "sha",
            "sse",
            "sse2",
            "sse3",
            "sse4.1",
            "sse4.2",
            "sse4a",
            "ssse3",
            "tbm",
            "vaes",
            "vpclmulqdq",
            "xsave",
            "xsavec",
            "xsaveopt",
            "xsaves",
            "crt-static",
            "16bit-mode",
            "32bit-mode",
            "3dnow",
            "3dnowa",
            "64bit",
            "64bit-mode",
            "allow-light-256-bit",
            "amx-bf16",
            "amx-fp16",
            "amx-int8",
            "amx-tile",
            "avx512fp16",
            "avxifma",
            "avxneconvert",
            "avxvnni",
            "avxvnniint8",
            "branchfusion",
            "cldemote",
            "clflushopt",
            "clwb",
            "clzero",
            "cmov",
            "cmpccxadd",
            "crc32",
            "cx8",
            "enqcmd",
            "false-deps-getmant",
            "false-deps-lzcnt-tzcnt",
            "false-deps-mulc",
            "false-deps-mullq",
            "false-deps-perm",
            "false-deps-popcnt",
            "false-deps-range",
            "fast-11bytenop",
            "fast-15bytenop",
            "fast-7bytenop",
            "fast-bextr",
            "fast-gather",
            "fast-hops",
            "fast-lzcnt",
            "fast-movbe",
            "fast-scalar-fsqrt",
            "fast-scalar-shift-masks",
            "fast-shld-rotate",
            "fast-variable-crosslane-shuffle",
            "fast-variable-perlane-shuffle",
            "fast-vector-fsqrt",
            "fast-vector-shift-masks",
            "fma4",
            "fsgsbase",
            "fsrm",
            "harden-sls-ijmp",
            "harden-sls-ret",
            "hreset",
            "idivl-to-divb",
            "idivq-to-divl",
            "invpcid",
            "kl",
            "lea-sp",
            "lea-uses-ag",
            "lvi-cfi",
            "lvi-load-hardening",
            "lwp",
            "macrofusion",
            "mmx",
            "movdir64b",
            "movdiri",
            "mwaitx",
            "nopl",
            "pad-short-functions",
            "pconfig",
            "pku",
            "prefer-128-bit",
            "prefer-256-bit",
            "prefer-mask-registers",
            "prefetchi",
            "prefetchwt1",
            "prfchw",
            "ptwrite",
            "raoint",
            "rdpid",
            "rdpru",
            "retpoline",
            "retpoline-external-thunk",
            "retpoline-indirect-branches",
            "retpoline-indirect-calls",
            "sahf",
            "sbb-dep-breaking",
            "serialize",
            "seses",
            "sgx",
            "shstk",
            "slow-3ops-lea",
            "slow-incdec",
            "slow-lea",
            "slow-pmaddwd",
            "slow-pmulld",
            "slow-shld",
            "slow-two-mem-ops",
            "slow-unaligned-mem-16",
            "slow-unaligned-mem-32",
            "soft-float",
            "sse-unaligned-mem",
            "tagged-globals",
            "tsxldtrk",
            "uintr",
            "use-glm-div-sqrt-costs",
            "use-slm-arith-costs",
            "vzeroupper",
            "waitpkg",
            "wbnoinvd",
            "widekl",
            "x87",
            "xop"
        );
    } else if cfg!(target_arch = "aarch64") {
        print_if_feature_enabled!(
            "aes",
            "bf16",
            "bti",
            "crc",
            "dit",
            "dotprod",
            "dpb",
            "dpb2",
            "f32mm",
            "f64mm",
            "fcma",
            "fhm",
            "flagm",
            "fp16",
            "frintts",
            "i8mm",
            "jsconv",
            "lor",
            "lse",
            "mte",
            "neon",
            "paca",
            "pacg",
            "pan",
            "pmuv3",
            "rand",
            "ras",
            "rcpc",
            "rcpc2",
            "rdm",
            "sb",
            "sha2",
            "sha3",
            "sm4",
            "spe",
            "ssbs",
            "sve",
            "sve2",
            "sve2-aes",
            "sve2-bitperm",
            "sve2-sha3",
            "sve2-sm4",
            "tme",
            "v8.1a",
            "v8.2a",
            "v8.3a",
            "v8.4a",
            "v8.5a",
            "v8.6a",
            "v8.7a",
            "vh",
            "crt-static",
            "a35",
            "a510",
            "a53",
            "a55",
            "a57",
            "a64fx",
            "a65",
            "a710",
            "a715",
            "a72",
            "a73",
            "a75",
            "a76",
            "a77",
            "a78",
            "a78c",
            "aggressive-fma",
            "all",
            "alternate-sextload-cvt-f32-pattern",
            "altnzcv",
            "am",
            "ampere1",
            "ampere1a",
            "amvs",
            "apple-a10",
            "apple-a11",
            "apple-a12",
            "apple-a13",
            "apple-a14",
            "apple-a15",
            "apple-a16",
            "apple-a7",
            "apple-a7-sysreg",
            "arith-bcc-fusion",
            "arith-cbz-fusion",
            "ascend-store-address",
            "b16b16",
            "balance-fp-ops",
            "brbe",
            "call-saved-x10",
            "call-saved-x11",
            "call-saved-x12",
            "call-saved-x13",
            "call-saved-x14",
            "call-saved-x15",
            "call-saved-x18",
            "call-saved-x8",
            "call-saved-x9",
            "carmel",
            "ccidx",
            "clrbhb",
            "cmp-bcc-fusion",
            "cortex-r82",
            "cortex-x1",
            "cortex-x2",
            "cortex-x3",
            "crypto",
            "cssc",
            "custom-cheap-as-move",
            "d128",
            "disable-latency-sched-heuristic",
            "ecv",
            "el2vmsa",
            "el3",
            "enable-select-opt",
            "ete",
            "exynos-cheap-as-move",
            "exynosm3",
            "exynosm4",
            "falkor",
            "fgt",
            "fix-cortex-a53-835769",
            "fmv",
            "force-32bit-jump-tables",
            "fp-armv8",
            "fuse-address",
            "fuse-adrp-add",
            "fuse-aes",
            "fuse-arith-logic",
            "fuse-crypto-eor",
            "fuse-csel",
            "fuse-literals",
            "harden-sls-blr",
            "harden-sls-nocomdat",
            "harden-sls-retbr",
            "hbc",
            "hcx",
            "ite",
            "kryo",
            "ls64",
            "lse128",
            "lse2",
            "lsl-fast",
            "mec",
            "mops",
            "mpam",
            "neoverse512tvb",
            "neoversee1",
            "neoversen1",
            "neoversen2",
            "neoversev1",
            "neoversev2",
            "nmi",
            "no-bti-at-return-twice",
            "no-neg-immediates",
            "no-zcz-fp",
            "nv",
            "outline-atomics",
            "pan-rwv",
            "predictable-select-expensive",
            "predres",
            "prfm-slc-target",
            "rasv2",
            "rcpc3",
            "reserve-x1",
            "reserve-x10",
            "reserve-x11",
            "reserve-x12",
            "reserve-x13",
            "reserve-x14",
            "reserve-x15",
            "reserve-x18",
            "reserve-x2",
            "reserve-x20",
            "reserve-x21",
            "reserve-x22",
            "reserve-x23",
            "reserve-x24",
            "reserve-x25",
            "reserve-x26",
            "reserve-x27",
            "reserve-x28",
            "reserve-x3",
            "reserve-x30",
            "reserve-x4",
            "reserve-x5",
            "reserve-x6",
            "reserve-x7",
            "reserve-x9",
            "rme",
            "saphira",
            "sel2",
            "slow-misaligned-128store",
            "slow-paired-128",
            "slow-strqro-store",
            "sme",
            "sme-f16f16",
            "sme-f64f64",
            "sme-i16i64",
            "sme2",
            "sme2p1",
            "spe-eef",
            "specres2",
            "specrestrict",
            "strict-align",
            "sve2p1",
            "tagged-globals",
            "the",
            "thunderx",
            "thunderx2t99",
            "thunderx3t110",
            "thunderxt81",
            "thunderxt83",
            "thunderxt88",
            "tlb-rmi",
            "tpidr-el1",
            "tpidr-el2",
            "tpidr-el3",
            "tracev8.4",
            "trbe",
            "tsv110",
            "uaops",
            "use-experimental-zeroing-pseudos",
            "use-postra-scheduler",
            "use-reciprocal-square-root",
            "use-scalar-inc-vl",
            "v8.8a",
            "v8.9a",
            "v8a",
            "v8r",
            "v9.1a",
            "v9.2a",
            "v9.3a",
            "v9.4a",
            "v9a",
            "wfxt",
            "xs",
            "zcm",
            "zcz",
            "zcz-fp-workaround",
            "zcz-gp"
        );
    }
}