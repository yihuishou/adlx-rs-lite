#![doc = include_str!("../README.md")]

#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    dead_code,
    // bindgen 生成的常量数组尺寸很大（如 ADLX_3DLUT_Data），clippy 1.98 起会报 large_const_arrays
    clippy::large_const_arrays
)]
pub mod ffi;

pub mod display;
pub mod display_free_sync;
pub mod display_list;
pub mod display_services;
pub mod gpu;
pub mod gpu_list;
pub mod gpu_metrics;
pub mod gpu_stress_test;
pub mod helper;
pub mod interface;
pub mod list;
pub mod performance_monitoring_services;
pub mod result;
pub mod system;
pub mod three_d_chill;
pub mod three_d_frame_rate_target_control;
pub mod three_d_settings_services;
pub mod three_d_wait_for_vertical_refresh;
pub mod variable_graphics_memory;
pub mod version;

pub use display::*;
pub use display_free_sync::*;
pub use display_list::*;
pub use display_services::*;
pub use gpu::*;
pub use gpu_list::*;
pub use gpu_metrics::*;
pub use gpu_stress_test::*;
pub use helper::*;
pub use interface::*;
pub use list::*;
pub use performance_monitoring_services::*;
pub use result::*;
pub use system::*;
pub use three_d_chill::*;
pub use three_d_frame_rate_target_control::*;
pub use three_d_settings_services::*;
pub use three_d_wait_for_vertical_refresh::*;
pub use variable_graphics_memory::*;
pub use version::*;
