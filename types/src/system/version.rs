use crate::Timestamp;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum GoOS {
    Aix,
    Android,
    Darwin,
    Dragonfly,
    Freebsd,
    Hurd,
    Illumos,
    Ios,
    Js,
    Linux,
    Nacl,
    Netbsd,
    Openbsd,
    Plan9,
    Solaris,
    Wasip1,
    Windows,
    Zos,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum GoArch {
    A386,
    Amd64,
    Amd64p32,
    Arm,
    Armbe,
    Arm64,
    Arm64be,
    Loong64,
    Mips,
    Mipsle,
    Mips64,
    Mips64le,
    Mips64p32,
    Mips64p32le,
    Ppc,
    Ppc64,
    Ppc64le,
    Riscv,
    Riscv64,
    S390,
    S390x,
    Sparc,
    Sparc64,
    Wasm,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct VersionInfo {
    pub version: String,
    pub codename: String,
    pub long_version: String,
    pub extra: String,
    pub os: GoOS,
    pub arch: GoArch,
    pub is_beta: bool,
    pub is_candidate: bool,
    pub is_release: bool,
    pub date: Timestamp,
    #[serde(default)]
    pub tags: Vec<String>,
    pub stamp: String,
    pub user: String,
    pub container: bool,
}
