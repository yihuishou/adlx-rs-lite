//! ADLX 运行时版本跟踪与版本门控。
//!
//! ADLX 的 `amdadlx64.dll` 由 AMD 随显卡驱动分发，它的版本节奏与 SDK 的发布节奏并不同步。
//! 例如 ADLX 1.4.0.121 的驱动 DLL 里也实现了 `IADLXGPU3`，但只实现了 `MicroArchitecture`
//! 一个方法，其余方法在 vtable 中根本没有对应的槽位——直接调用会触发访问冲突
//! （`0xC0000005`），而不是返回一个错误码。
//!
//! 因此 [`crate::gpu::Gpu3`] 这类接口在调用前必须先确认运行时版本。

use std::sync::OnceLock;

use super::result::{Error, Result};

/// ADLX 运行时版本号。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AdlxVersion {
    /// 主版本号
    pub major: u32,
    /// 次版本号
    pub minor: u32,
    /// 发布号
    pub release: u32,
    /// 构建号
    pub build: u32,
}

impl AdlxVersion {
    /// 从 `ADLXQueryFullVersion` 返回的完整版本号中解析。
    ///
    /// 版本号的位布局与 [`crate::helper::AdlxHelper`] 构造 `HEADER_VERSION` 时一致：
    /// `major << 48 | minor << 32 | release << 16 | build`。
    pub fn from_full_version(full_version: u64) -> Self {
        Self {
            major: ((full_version >> 48) & 0xFFFF) as u32,
            minor: ((full_version >> 32) & 0xFFFF) as u32,
            release: ((full_version >> 16) & 0xFFFF) as u32,
            build: (full_version & 0xFFFF) as u32,
        }
    }

    /// 本版本是否不低于 `(major, minor, release)`。
    pub fn at_least(&self, major: u32, minor: u32, release: u32) -> bool {
        (self.major, self.minor, self.release) >= (major, minor, release)
    }
}

impl std::fmt::Display for AdlxVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.release, self.build
        )
    }
}

static RUNTIME_VERSION: OnceLock<AdlxVersion> = OnceLock::new();

/// 记录当前加载的 ADLX 运行时版本。
pub(crate) fn set_runtime_version(version: AdlxVersion) {
    // 版本只会被记录一次，重复初始化时保留第一个值。
    let _ = RUNTIME_VERSION.set(version);
}

/// 当前加载的 ADLX 运行时版本。
///
/// 只有在 [`crate::helper::AdlxHelper::new()`] 成功之后才有值；
/// 如果 ADLX 是用别的方式初始化的，则返回 `None`。
pub fn runtime_version() -> Option<AdlxVersion> {
    RUNTIME_VERSION.get().copied()
}

/// 确保运行时版本不低于要求的版本，否则返回 `NOT_SUPPORTED` 错误。
pub(crate) fn ensure_version_at_least(major: u32, minor: u32, release: u32) -> Result<()> {
    match runtime_version() {
        Some(version) if version.at_least(major, minor, release) => Ok(()),
        _ => Err(Error::not_supported()),
    }
}
