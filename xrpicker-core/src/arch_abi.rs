// Copyright 2022-2025, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fmt::Display;

use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
#[allow(non_camel_case_types)]
pub enum RuntimeArchAbi {
    /// 64-bit x86 instructions, using an ILP32 model (32-bit pointers)
    Arch_x32,
    /// 64-bit x86
    Arch_x86_64,
    /// 32-bit x86
    Arch_i686,
    /// 64-bit ARM architecture, little endian
    Arch_aarch64,
    /// 32-bit ARMv7-A architecture, little endian, with hardware floating point and VFP PCS ABI
    Arch_armv7a_vfp,
    /// 32-bit ARMv5TE architecture or compatible, little endian
    Arch_armv5te,
    /// 64-bit MIPS architecture, little endian
    Arch_mips64,
    /// 32-bit MIPS architecture, little endian
    Arch_mips,
    /// 64-bit PowerPC architecture, big endian
    Arch_ppc64,
    /// 64-bit POWER8/POWER9 architecture, little endian (OpenPOWER ELF ABI v2)
    Arch_ppc64el,
    /// 64-bit S390/z-Series architecture, big endian
    Arch_s390x,
    /// 32-bit HP PA-RISC architecture, big endian
    Arch_hppa,
    /// 64-bit Alpha architecture
    Arch_alpha,
    /// 64-bit IA-64 architecture
    Arch_ia64,
    /// 32-bit Motorola 68000-based architecture, big endian
    Arch_m68k,
    /// 64-bit RISC-V architecture, little endian
    Arch_riscv64,
    /// 64-bit SPARC architecture
    Arch_sparc64,
    /// 64-bit LoongArch architecture, little endian (LP64D ABI)
    Arch_loongarch64,
}

impl Display for RuntimeArchAbi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeArchAbi::Arch_x32 => f.write_str("x32"),
            RuntimeArchAbi::Arch_x86_64 => f.write_str("x86_64"),
            RuntimeArchAbi::Arch_i686 => f.write_str("i686"),
            RuntimeArchAbi::Arch_aarch64 => f.write_str("aarch64"),
            RuntimeArchAbi::Arch_armv7a_vfp => f.write_str("armv7a-vfp"),
            RuntimeArchAbi::Arch_armv5te => f.write_str("armv5te"),
            RuntimeArchAbi::Arch_mips64 => f.write_str("mips64"),
            RuntimeArchAbi::Arch_mips => f.write_str("mips"),
            RuntimeArchAbi::Arch_ppc64 => f.write_str("ppc64"),
            RuntimeArchAbi::Arch_ppc64el => f.write_str("ppc64el"),
            RuntimeArchAbi::Arch_s390x => f.write_str("s390x"),
            RuntimeArchAbi::Arch_hppa => f.write_str("hppa"),
            RuntimeArchAbi::Arch_alpha => f.write_str("alpha"),
            RuntimeArchAbi::Arch_ia64 => f.write_str("ia64"),
            RuntimeArchAbi::Arch_m68k => f.write_str("m68k"),
            RuntimeArchAbi::Arch_riscv64 => f.write_str("riscv64"),
            RuntimeArchAbi::Arch_sparc64 => f.write_str("sparc64"),
            RuntimeArchAbi::Arch_loongarch64 => f.write_str("loongarch64"),
        }
    }
}

impl RuntimeArchAbi {
    pub fn filename_suffix(self) -> &'static str {
        match self {
            RuntimeArchAbi::Arch_x32 => ".x32.json",
            RuntimeArchAbi::Arch_x86_64 => ".x86_64.json",
            RuntimeArchAbi::Arch_i686 => ".i686.json",
            RuntimeArchAbi::Arch_aarch64 => ".aarch64.json",
            RuntimeArchAbi::Arch_armv7a_vfp => ".armv7a-vfp.json",
            RuntimeArchAbi::Arch_armv5te => ".armv5te.json",
            RuntimeArchAbi::Arch_mips64 => ".mips64.json",
            RuntimeArchAbi::Arch_mips => ".mips.json",
            RuntimeArchAbi::Arch_ppc64 => ".ppc64.json",
            RuntimeArchAbi::Arch_ppc64el => ".ppc64el.json",
            RuntimeArchAbi::Arch_s390x => ".s390x.json",
            RuntimeArchAbi::Arch_hppa => ".hppa.json",
            RuntimeArchAbi::Arch_alpha => ".alpha.json",
            RuntimeArchAbi::Arch_ia64 => ".ia64.json",
            RuntimeArchAbi::Arch_m68k => ".m68k.json",
            RuntimeArchAbi::Arch_riscv64 => ".riscv64.json",
            RuntimeArchAbi::Arch_sparc64 => ".sparc64.json",
            RuntimeArchAbi::Arch_loongarch64 => ".loongarch64.json",
        }
    }

    #[allow(unreachable_code)]
    pub fn get_current_arch() -> Option<RuntimeArchAbi> {
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        return Some(RuntimeArchAbi::Arch_x32);
        #[cfg(target_arch = "x86_64")]
        return Some(RuntimeArchAbi::Arch_x86_64);
        #[cfg(target_arch = "x86")]
        return Some(RuntimeArchAbi::Arch_i686);
        #[cfg(target_arch = "aarch64")]
        return Some(RuntimeArchAbi::Arch_aarch64);
        // TODO wrong on Android but this tool is not useful there.
        #[cfg(all(target_arch = "arm", target_abi = "eabihf"))]
        return Some(RuntimeArchAbi::Arch_armv7a_vfp);
        #[cfg(all(target_arch = "arm", target_abi = "eabi"))]
        return Some(RuntimeArchAbi::Arch_armv5te);
        #[cfg(target_arch = "mips64")]
        return Some(RuntimeArchAbi::Arch_mips64);
        #[cfg(target_arch = "mips")]
        return Some(RuntimeArchAbi::Arch_mips);
        #[cfg(all(target_arch = "powerpc64", target_endian = "big"))]
        return Some(RuntimeArchAbi::Arch_ppc64);
        #[cfg(all(target_arch = "powerpc64", target_endian = "little"))]
        return Some(RuntimeArchAbi::Arch_ppc64el);
        #[cfg(target_arch = "s390x")]
        return Some(RuntimeArchAbi::Arch_s390x);
        #[cfg(target_arch = "m68k")]
        return Some(RuntimeArchAbi::Arch_m68k);
        #[cfg(target_arch = "riscv64")]
        return Some(RuntimeArchAbi::Arch_riscv64);
        #[cfg(target_arch = "sparc64")]
        return Some(RuntimeArchAbi::Arch_sparc64);
        #[cfg(target_arch = "loongarch64")]
        return Some(RuntimeArchAbi::Arch_loongarch64);
        None
    }
}

/// The defined architecture and ABI identifiers used to decorate active runtime manifest filenames on some platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ManifestArchDecoration {
    /// Architecture not indicated in filename
    Unspecified,
    /// Single architecture specified in filename
    Specified(RuntimeArchAbi),
}

impl ManifestArchDecoration {
    /// Get the decorated runtime filename suffix corresponding to a given architecture.
    pub fn filename_suffix(self) -> &'static str {
        match self {
            ManifestArchDecoration::Unspecified => ".json",
            ManifestArchDecoration::Specified(arch) => arch.filename_suffix(),
        }
    }
}

impl From<Option<RuntimeArchAbi>> for ManifestArchDecoration {
    fn from(value: Option<RuntimeArchAbi>) -> Self {
        match value {
            Some(value) => ManifestArchDecoration::Specified(value),
            None => ManifestArchDecoration::Unspecified,
        }
    }
}
