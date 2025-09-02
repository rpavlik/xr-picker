// Copyright 2022-2025, Collabora, Ltd.
// SPDX-License-Identifier: MIT OR Apache-2.0

use strum::EnumIter;

/// The defined architecture and ABI identifiers used to decorate active runtime manifest filenames on some platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ManifestArchDecoration {
    /// Architecture not indicated in filename
    Unspecified,
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

impl ManifestArchDecoration {
    /// Get the decorated runtime filename suffix corresponding to a given architecture.
    pub fn get_filename_suffix(self) -> &'static str {
        match self {
            ManifestArchDecoration::Unspecified => ".json",
            ManifestArchDecoration::Arch_x32 => ".x32.json",
            ManifestArchDecoration::Arch_x86_64 => ".x86_64.json",
            ManifestArchDecoration::Arch_i686 => ".i686.json",
            ManifestArchDecoration::Arch_aarch64 => ".aarch64.json",
            ManifestArchDecoration::Arch_armv7a_vfp => ".armv7a-vfp.json",
            ManifestArchDecoration::Arch_armv5te => ".armv5te.json",
            ManifestArchDecoration::Arch_mips64 => ".mips64.json",
            ManifestArchDecoration::Arch_mips => ".mips.json",
            ManifestArchDecoration::Arch_ppc64 => ".ppc64.json",
            ManifestArchDecoration::Arch_ppc64el => ".ppc64el.json",
            ManifestArchDecoration::Arch_s390x => ".s390x.json",
            ManifestArchDecoration::Arch_hppa => ".hppa.json",
            ManifestArchDecoration::Arch_alpha => ".alpha.json",
            ManifestArchDecoration::Arch_ia64 => ".ia64.json",
            ManifestArchDecoration::Arch_m68k => ".m68k.json",
            ManifestArchDecoration::Arch_riscv64 => ".riscv64.json",
            ManifestArchDecoration::Arch_sparc64 => ".sparc64.json",
            ManifestArchDecoration::Arch_loongarch64 => ".loongarch64.json",
        }
    }

    #[allow(unreachable_code)]
    pub fn get_current_arch() -> Option<ManifestArchDecoration> {
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        return Some(ManifestArchDecoration::Arch_x32);
        #[cfg(target_arch = "x86_64")]
        return Some(ManifestArchDecoration::Arch_x86_64);
        #[cfg(target_arch = "x86")]
        return Some(ManifestArchDecoration::Arch_i686);
        #[cfg(target_arch = "aarch64")]
        return Some(ManifestArchDecoration::Arch_aarch64);
        // TODO wrong on Android but this tool is not useful there.
        #[cfg(all(target_arch = "arm", target_abi = "eabihf"))]
        return Some(ManifestArchDecoration::Arch_armv7a_vfp);
        #[cfg(all(target_arch = "arm", target_abi = "eabi"))]
        return Some(ManifestArchDecoration::Arch_armv5te);
        #[cfg(target_arch = "mips64")]
        return Some(ManifestArchDecoration::Arch_mips64);
        #[cfg(target_arch = "mips")]
        return Some(ManifestArchDecoration::Arch_mips);
        #[cfg(all(target_arch = "powerpc64", target_endian = "big"))]
        return Some(ManifestArchDecoration::Arch_ppc64);
        #[cfg(all(target_arch = "powerpc64", target_endian = "little"))]
        return Some(ManifestArchDecoration::Arch_ppc64el);
        #[cfg(target_arch = "s390x")]
        return Some(ManifestArchDecoration::Arch_s390x);
        #[cfg(target_arch = "m68k")]
        return Some(ManifestArchDecoration::Arch_m68k);
        #[cfg(target_arch = "riscv64")]
        return Some(ManifestArchDecoration::Arch_riscv64);
        #[cfg(target_arch = "sparc64")]
        return Some(ManifestArchDecoration::Arch_sparc64);
        #[cfg(target_arch = "loongarch64")]
        return Some(ManifestArchDecoration::Arch_loongarch64);
        None
    }
}

impl From<Option<RuntimeArchAbi>> for ManifestArchDecoration {
    fn from(value: Option<RuntimeArchAbi>) -> Self {
        match value {
            Some(value) => ManifestArchDecoration::from(value),
            None => ManifestArchDecoration::Unspecified,
        }
    }
}

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

impl From<RuntimeArchAbi> for ManifestArchDecoration {
    fn from(value: RuntimeArchAbi) -> Self {
        match value {
            RuntimeArchAbi::Arch_x32 => ManifestArchDecoration::Arch_x32,
            RuntimeArchAbi::Arch_x86_64 => ManifestArchDecoration::Arch_x86_64,
            RuntimeArchAbi::Arch_i686 => ManifestArchDecoration::Arch_i686,
            RuntimeArchAbi::Arch_aarch64 => ManifestArchDecoration::Arch_aarch64,
            RuntimeArchAbi::Arch_armv7a_vfp => ManifestArchDecoration::Arch_armv7a_vfp,
            RuntimeArchAbi::Arch_armv5te => ManifestArchDecoration::Arch_armv5te,
            RuntimeArchAbi::Arch_mips64 => ManifestArchDecoration::Arch_mips64,
            RuntimeArchAbi::Arch_mips => ManifestArchDecoration::Arch_mips,
            RuntimeArchAbi::Arch_ppc64 => ManifestArchDecoration::Arch_ppc64,
            RuntimeArchAbi::Arch_ppc64el => ManifestArchDecoration::Arch_ppc64el,
            RuntimeArchAbi::Arch_s390x => ManifestArchDecoration::Arch_s390x,
            RuntimeArchAbi::Arch_hppa => ManifestArchDecoration::Arch_hppa,
            RuntimeArchAbi::Arch_alpha => ManifestArchDecoration::Arch_alpha,
            RuntimeArchAbi::Arch_ia64 => ManifestArchDecoration::Arch_ia64,
            RuntimeArchAbi::Arch_m68k => ManifestArchDecoration::Arch_m68k,
            RuntimeArchAbi::Arch_riscv64 => ManifestArchDecoration::Arch_riscv64,
            RuntimeArchAbi::Arch_sparc64 => ManifestArchDecoration::Arch_sparc64,
            RuntimeArchAbi::Arch_loongarch64 => ManifestArchDecoration::Arch_loongarch64,
        }
    }
}
