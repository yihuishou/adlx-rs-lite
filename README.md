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
adlx = "1.0.0"
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

## Wrapped API

Safe Rust wrappers exist for **28 of the 165** ADLX interfaces. The remaining 137 are
reachable only through the raw `ffi` module, which requires driving the COM-style vtables
by hand. Interfaces marked **1.5** are version-gated and return `NOT_SUPPORTED` on an
older driver instead of crashing.

### Entry point

| Type | ADLX interface | Methods |
| --- | --- | --- |
| `AdlxHelper` | — | `new()`, `system()`, `full_version()`, `version()` |
| `System` | `IADLXSystem` | `gpus()`, `get_displays_services()`, `get_3d_settings_services()`, `performance_monitoring_services()`, `hybrid_graphics_type()`, `cast::<T>()` |
| `System1` | `IADLXSystem1` | no methods yet — `power_tuning_services()` is stubbed out |
| `System3` | `IADLXSystem3` | `variable_graphics_memory()` — **1.5** |

### Core

| Type | ADLX interface | Methods |
| --- | --- | --- |
| `InterfaceImpl` | `IADLXInterface` | `cast::<T>()`, `from_raw()`, `into_raw()`, `as_raw()` |
| `List` | `IADLXList` | `size()` |

### GPU

| Type | ADLX interface | Methods |
| --- | --- | --- |
| `Gpu` | `IADLXGPU` | `name()`, `vendor_id()`, `driver_path()`, `pnp_string()`, `device_id()`, `revision_id()`, `sub_system_id()`, `sub_system_vendor_id()`, `unique_id()`, `pci_address_from_unique_id()`, `asic_family_type()`, `type_()`, `is_external()`, `has_desktops()`, `total_vram()`, `vram_type()` |
| `Gpu1` | `IADLXGPU1` | `pci_bus_type()`, `pci_bus_lane_width()`, `multi_gpu_mode()`, `product_name()` — derefs to `Gpu` |
| `Gpu2` | `IADLXGPU2` | `is_power_off()`, `power_on()`, `abort_power_off()`, `is_supported_application_list()`, `amd_software_release_date()`, `amd_software_edition()`, `amd_software_version()`, `driver_version()`, `amd_windows_driver_version()`, `luid()` — derefs to `Gpu1` |
| `Gpu3` | `IADLXGPU3` | `micro_architecture()`, `highest_vram_bandwidth()`, `invisible_vram()`, `visible_vram()`, `vram_vendor_rev_id()`, `vram_bandwidth()`, `vram_bit_rate()`, `is_supported_stress_test()`, `start_stress_test()` — derefs to `Gpu2`, **1.5** |
| `GpuList` | `IADLXGPUList` | `at()`, `add_back()`, `iter()` |
| `GpuStressTestFinishedListener` | `IADLXGPUStressTestFinishedListener` | `new(callback)` — **1.5** |

### Performance monitoring

| Type | ADLX interface | Methods |
| --- | --- | --- |
| `PerformanceMonitoringServices` | `IADLXPerformanceMonitoringServices` | `current_gpu_metrics()`, `supported_gpu_metrics()`, `gpu_metrics_history()`, `sampling_interval_range()`, `set_sampling_interval()`, `sampling_interval()`, `start_performance_tracking()`, `stop_performance_tracking()`, `clear_performance_metrics_history()`, `max_performance_metrics_history_range()`, `set_max_performance_metrics_history_size()`, `max_performance_metrics_history_size()`, `current_performance_metrics_history_size_in_sec()` |
| `GpuMetrics` | `IADLXGPUMetrics` | `time_stamp()`, `usage()`, `clock_speed()`, `vram_clock_speed()`, `temperature()`, `hotspot_temperature()`, `intake_temperature()`, `power()`, `total_board_power()`, `voltage()`, `fan_speed()`, `vram()` |
| `GpuMetrics3` | `IADLXGPUMetrics3` | `fan_duty()` — **1.5** |
| `GpuMetricsSupport` | `IADLXGPUMetricsSupport` | `is_supported_gpu_*()` and `gpu_*_range()` for usage, clock speed, VRAM clock speed, temperature, hotspot temperature, intake temperature, power, total board power, fan speed, VRAM and voltage |
| `GpuMetricsSupport3` | `IADLXGPUMetricsSupport3` | `is_supported_gpu_fan_duty()`, `gpu_fan_duty_range()` — **1.5** |
| `GpuMetricsList` | `IADLXGPUMetricsList` | `at()`, `add_back()` |

Metrics must be checked before they are read — not every metric is supported on every
GPU/driver combination:

```rust,no_run
use adlx::helper::AdlxHelper;

fn main() -> anyhow::Result<()> {
    let helper = AdlxHelper::new()?;
    let system = helper.system();
    let gpus = system.gpus()?;
    let gpu = gpus.at(0)?;

    let pms = system.performance_monitoring_services()?;
    let metrics = pms.current_gpu_metrics(&gpu)?;
    let support = pms.supported_gpu_metrics(&gpu)?;

    if support.is_supported_gpu_total_board_power()? {
        println!("total board power: {} W", metrics.total_board_power()?);
    }

    Ok(())
}
```

### Display

| Type | ADLX interface | Methods |
| --- | --- | --- |
| `Display` | `IADLXDisplay` | `get_gpu()` |
| `DisplayList` | `IADLXDisplayList` | `at()`, `iter()` |
| `DisplayServices` | `IADLXDisplayServices` | `get_displays()`, `get_free_sync()` |
| `DisplayFreeSync` | `IADLXDisplayFreeSync` | `is_supported()`, `is_enabled()` |

### 3D settings

| Type | ADLX interface | Methods |
| --- | --- | --- |
| `ThreeDSettingsServices` | `IADLX3DSettingsServices` | `get_wait_for_vertical_refresh()`, `get_chill()`, `get_frame_rate_target_control()` |
| `ThreeDWaitForVerticalRefresh` | `IADLX3DWaitForVerticalRefresh` | `is_supported()`, `is_enabled()`, `get_mode()`, `set_mode()` |
| `ThreeDChill` | `IADLX3DChill` | `is_supported()`, `is_enabled()`, `get_fps_range()`, `get_min_fps()`, `get_max_fps()` |
| `ThreeDFrameRateTargetControl` | `IADLX3DFrameRateTargetControl` | `is_supported()`, `is_enabled()`, `get_fps_range()`, `get_fps()` |

### Variable graphics memory (1.5)

| Type | ADLX interface | Methods |
| --- | --- | --- |
| `VariableGraphicsMemory` | `IADLXVariableGraphicsMemory` | `is_supported()`, `option()`, `default_option()`, `available_options()`, `set_option()` |
| `VariableGraphicsMemoryOption` | `IADLXVariableGraphicsMemoryOption` | `name()`, `mode()`, `memory_carved()`, `memory_remaining()` |
| `VariableGraphicsMemoryOptionList` | `IADLXVariableGraphicsMemoryOptionList` | `size()`, `at()` |

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
