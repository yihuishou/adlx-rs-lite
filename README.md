# ADLX

[![Actions Status](https://github.com/Traverse-Research/adlx-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Traverse-Research/adlx-rs/actions)
[![Latest version](https://img.shields.io/crates/v/adlx.svg?logo=rust)](https://crates.io/crates/adlx)
[![Documentation](https://docs.rs/adlx/badge.svg)](https://docs.rs/adlx)
[![MSRV](https://img.shields.io/badge/rustc-1.74.0+-ab6000.svg)](https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html)
[![Lines of code](https://tokei.rs/b1/github/Traverse-Research/adlx-rs)](https://github.com/Traverse-Research/adlx-rs)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4%20adopted-ff69b4.svg)](./CODE_OF_CONDUCT.md)

[![Banner](banner.png)](https://traverseresearch.nl)

Bindings for [AMD's Device Library eXtra (ADLX)](https://gpuopen.com/adlx/).

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
adlx = "0.0.0-alpha.1"
```

## ADLX version support

The bindings are generated from the ADLX SDK **v1.5** and cover all 165 SDK interfaces.
`amdadlx64.dll` is loaded at runtime from the system directory — it ships with the AMD
display driver, so the installed driver is often older than the SDK.

Interfaces introduced in ADLX 1.5 are version-gated: on an older driver they return
`NOT_SUPPORTED` instead of crashing.

| Interface | Since |
| --- | --- |
| `IADLXGPU3` (micro architecture, VRAM details, stress test) | ADLX 1.5 |
| `IADLXSystem3` → variable graphics memory | ADLX 1.5 |
| `IADLXGPUMetrics3` / `IADLXGPUMetricsSupport3` (fan duty) | ADLX 1.5 |
| `IADLXVariableGraphicsMemory` | ADLX 1.5 |

The runtime version is available via `adlx::runtime_version()`.

## Code example

```rust,no_run
use adlx::{gpu::Gpu1, helper::AdlxHelper, interface::Interface};
use anyhow::Result;

fn main() -> Result<()> {
    let helper = AdlxHelper::new()?;
    let system = helper.system();
    let gpu_list = system.gpus()?;

    for gpu in 0..gpu_list.size() {
        let gpu = gpu_list.at(gpu)?;
        let gpu1 = gpu.cast::<Gpu1>()?;
        dbg!(gpu1.name()?);
        dbg!(gpu1.product_name()?);
    }

    Ok(())
}
```
