#[cfg(target_arch = "aarch64")]
use libc::{
    AT_HWCAP, AT_HWCAP2, HWCAP_AES, HWCAP_ASIMD, HWCAP_ASIMDDP, HWCAP_ASIMDFHM, HWCAP_ASIMDHP,
    HWCAP_ASIMDRDM, HWCAP_ATOMICS, HWCAP_CPUID, HWCAP_CRC32, HWCAP_DCPOP, HWCAP_DIT, HWCAP_EVTSTRM,
    HWCAP_FCMA, HWCAP_FLAGM, HWCAP_FP, HWCAP_FPHP, HWCAP_ILRCPC, HWCAP_JSCVT, HWCAP_LRCPC,
    HWCAP_PACA, HWCAP_PACG, HWCAP_PMULL, HWCAP_SB, HWCAP_SHA1, HWCAP_SHA2, HWCAP_SHA3,
    HWCAP_SHA512, HWCAP_SM3, HWCAP_SM4, HWCAP_SSBS, HWCAP_SVE, HWCAP_USCAT,
};

fn main() {
    #[cfg(target_arch = "aarch64")]
    {
        let hwcaps: u64 = unsafe { libc::getauxval(AT_HWCAP) };
        println!("hwcaps {:b}", hwcaps);
        println!("HWCAP_FP {}", hwcaps & HWCAP_FP);
        println!("HWCAP_ASIMD {}", hwcaps & HWCAP_ASIMD);
        println!("HWCAP_EVTSTRM {}", hwcaps & HWCAP_EVTSTRM);
        println!("HWCAP_AES {}", hwcaps & HWCAP_AES);
        println!("HWCAP_PMULL {}", hwcaps & HWCAP_PMULL);
        println!("HWCAP_SHA1 {}", hwcaps & HWCAP_SHA1);
        println!("HWCAP_SHA2 {}", hwcaps & HWCAP_SHA2);
        println!("HWCAP_CRC32 {}", hwcaps & HWCAP_CRC32);
        println!("HWCAP_ATOMICS {}", hwcaps & HWCAP_ATOMICS);
        println!("HWCAP_FPHP {}", hwcaps & HWCAP_FPHP);
        println!("HWCAP_ASIMDHP {}", hwcaps & HWCAP_ASIMDHP);
        println!("HWCAP_CPUID {}", hwcaps & HWCAP_CPUID);
        println!("HWCAP_ASIMDRDM {}", hwcaps & HWCAP_ASIMDRDM);
        println!("HWCAP_JSCVT {}", hwcaps & HWCAP_JSCVT);
        println!("HWCAP_FCMA {}", hwcaps & HWCAP_FCMA);
        println!("HWCAP_LRCPC {}", hwcaps & HWCAP_LRCPC);
        println!("HWCAP_DCPOP {}", hwcaps & HWCAP_DCPOP);
        println!("HWCAP_SHA3 {}", hwcaps & HWCAP_SHA3);
        println!("HWCAP_SM3 {}", hwcaps & HWCAP_SM3);
        println!("HWCAP_SM4 {}", hwcaps & HWCAP_SM4);
        println!("HWCAP_ASIMDDP {}", hwcaps & HWCAP_ASIMDDP);
        println!("HWCAP_SHA512 {}", hwcaps & HWCAP_SHA512);
        println!("HWCAP_SVE {}", hwcaps & HWCAP_SVE);
        println!("HWCAP_ASIMDFHM {}", hwcaps & HWCAP_ASIMDFHM);
        println!("HWCAP_DIT {}", hwcaps & HWCAP_DIT);
        println!("HWCAP_USCAT {}", hwcaps & HWCAP_USCAT);
        println!("HWCAP_ILRCPC {}", hwcaps & HWCAP_ILRCPC);
        println!("HWCAP_FLAGM {}", hwcaps & HWCAP_FLAGM);
        println!("HWCAP_SSBS {}", hwcaps & HWCAP_SSBS);
        println!("HWCAP_SB {}", hwcaps & HWCAP_SB);
        println!("HWCAP_PACA {}", hwcaps & HWCAP_PACA);
        println!("HWCAP_PACG {}", hwcaps & HWCAP_PACG);

        let hwcaps: u64 = unsafe { libc::getauxval(AT_HWCAP2) };
        println!("{}", hwcaps);
    }
}
