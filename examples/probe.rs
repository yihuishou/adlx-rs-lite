//! ADLX 可读取信息的全量探测示例（只读，不做任何修改）

use adlx::{
    gpu::{Gpu, Gpu1, Gpu2},
    helper::AdlxHelper,
    interface::Interface,
};
use anyhow::Result;

fn main() -> Result<()> {
    let helper = AdlxHelper::new()?;
    let system = helper.system();

    println!("=== ADLX 版本 ===");
    println!("  version        = {}", helper.version());
    println!("  full_version   = {:#x}", helper.full_version());
    println!(
        "  hybrid_graphics_type = {:?}",
        system.hybrid_graphics_type()?
    );

    // ---------- GPU 身份信息 ----------
    let gpu_list = system.gpus()?;
    println!("\n=== GPU 列表 (共 {} 个) ===", gpu_list.size());
    for i in 0..gpu_list.size() {
        let gpu: Gpu = gpu_list.at(i)?;
        println!("\n-- GPU #{i} --");
        println!("  name              = {:?}", gpu.name()?);
        println!("  vendor_id         = {:?}", gpu.vendor_id()?);
        println!("  type_             = {:?}", gpu.type_()?);
        println!("  asic_family_type  = {:?}", gpu.asic_family_type()?);
        println!("  is_external       = {:?}", gpu.is_external()?);
        println!("  has_desktops      = {:?}", gpu.has_desktops()?);
        println!("  total_vram (MB)   = {:?}", gpu.total_vram()?);
        println!("  vram_type         = {:?}", gpu.vram_type()?);
        println!("  device_id         = {:?}", gpu.device_id()?);
        println!("  revision_id       = {:?}", gpu.revision_id()?);
        println!("  sub_system_id     = {:?}", gpu.sub_system_id()?);
        println!("  sub_system_vendor = {:?}", gpu.sub_system_vendor_id()?);
        println!("  unique_id         = {:?}", gpu.unique_id()?);
        println!(
            "  pci_address       = {:?}",
            gpu.pci_address_from_unique_id()
        );
        println!("  driver_path       = {:?}", gpu.driver_path()?);
        println!("  pnp_string        = {:?}", gpu.pnp_string()?);

        // GPU1
        if let Ok(gpu1) = gpu.cast::<Gpu1>() {
            println!("  [GPU1] product_name     = {:?}", gpu1.product_name()?);
            println!("  [GPU1] pci_bus_type     = {:?}", gpu1.pci_bus_type()?);
            println!(
                "  [GPU1] pci_bus_lane_wid = {:?}",
                gpu1.pci_bus_lane_width()?
            );
            println!("  [GPU1] multi_gpu_mode   = {:?}", gpu1.multi_gpu_mode()?);
        }

        // GPU2
        if let Ok(gpu2) = gpu.cast::<Gpu2>() {
            println!(
                "  [GPU2] amd_software_edition    = {:?}",
                gpu2.amd_software_edition()?
            );
            println!(
                "  [GPU2] amd_software_version    = {:?}",
                gpu2.amd_software_version()?
            );
            println!(
                "  [GPU2] amd_software_release_dt = {:?}",
                gpu2.amd_software_release_date()?
            );
            println!(
                "  [GPU2] driver_version           = {:?}",
                gpu2.driver_version()?
            );
            println!(
                "  [GPU2] amd_windows_driver_ver   = {:?}",
                gpu2.amd_windows_driver_version()?
            );
            println!("  [GPU2] luid                     = {:?}", gpu2.luid()?);
            // 以下两个在部分驱动上会返回 NOT_SUPPORTED，用 .ok() 容错
            println!(
                "  [GPU2] is_power_off             = {:?}",
                gpu2.is_power_off().ok()
            );
            println!(
                "  [GPU2] is_supported_app_list    = {:?}",
                gpu2.is_supported_application_list().ok()
            );
        }
    }

    // ---------- 实时指标 ----------
    let pms = system.performance_monitoring_services()?;
    println!("\n=== 性能监控服务 ===");
    println!(
        "  sampling_interval          = {:?}",
        pms.sampling_interval()?
    );
    println!(
        "  sampling_interval_range    = {:?}",
        pms.sampling_interval_range()?
    );
    println!(
        "  max_history_size (sec)     = {:?}",
        pms.max_performance_metrics_history_size()?
    );
    println!(
        "  max_history_size_range     = {:?}",
        pms.max_performance_metrics_history_range()?
    );

    for i in 0..gpu_list.size() {
        let gpu = gpu_list.at(i)?;
        let m = pms.current_gpu_metrics(&gpu)?;
        let sup = pms.supported_gpu_metrics(&gpu)?;

        println!("\n-- GPU #{i} 实时指标 --");
        macro_rules! show {
            ($flag:expr, $val:expr, $label:literal, $unit:literal) => {
                if $flag? {
                    println!("  {:<24} = {:?} {}", $label, $val?, $unit);
                } else {
                    println!("  {:<24} = <不支持>", $label);
                }
            };
        }
        show!(sup.is_supported_gpu_usage(), m.usage(), "GPU 使用率", "%");
        show!(
            sup.is_supported_gpu_clock_speed(),
            m.clock_speed(),
            "核心频率",
            "MHz"
        );
        show!(
            sup.is_supported_gpu_vram_clock_speed(),
            m.vram_clock_speed(),
            "显存频率",
            "MHz"
        );
        show!(sup.is_supported_gpu_power(), m.power(), "功耗", "W");
        show!(
            sup.is_supported_gpu_total_board_power(),
            m.total_board_power(),
            "总板卡功耗",
            "W"
        );
        show!(sup.is_supported_gpu_voltage(), m.voltage(), "电压", "mV");
        show!(sup.is_supported_gpu_vram(), m.vram(), "显存占用", "MB");
        show!(
            sup.is_supported_gpu_fan_speed(),
            m.fan_speed(),
            "风扇转速",
            "RPM"
        );
        show!(
            sup.is_supported_gpu_temperature(),
            m.temperature(),
            "核心温度",
            "°C"
        );
        show!(
            sup.is_supported_gpu_intake_temperature(),
            m.intake_temperature(),
            "进气温度",
            "°C"
        );
        show!(
            sup.is_supported_gpu_hotspot_temperature(),
            m.hotspot_temperature(),
            "热点温度",
            "°C"
        );
        println!("  {:<24} = {}", "时间戳", m.time_stamp()?);

        println!("  -- 量程 --");
        println!("  usage_range        = {:?}", sup.gpu_usage_range().ok());
        println!(
            "  clock_speed_range  = {:?}",
            sup.gpu_clock_speed_range().ok()
        );
        println!(
            "  temperature_range  = {:?}",
            sup.gpu_temperature_range().ok()
        );
        println!("  power_range        = {:?}", sup.gpu_power_range().ok());
        println!(
            "  fan_speed_range    = {:?}",
            sup.gpu_fan_speed_range().ok()
        );
    }

    // ---------- 显示输出 ----------
    println!("\n=== 显示输出 ===");
    let ds = system.get_displays_services()?;
    let displays = ds.get_displays()?;
    println!("  显示器数量 = {}", displays.size());
    for i in 0..displays.size() {
        let display = displays.at(i)?;
        let fs = ds.get_free_sync(&display)?;
        println!(
            "  Display #{i}: FreeSync supported={} enabled={}",
            fs.is_supported()?,
            fs.is_enabled()?
        );
    }

    // ---------- 3D 设置状态 ----------
    println!("\n=== 3D / 帧率设置（只读状态）===");
    let tds = system.get_3d_settings_services()?;
    for i in 0..gpu_list.size() {
        let gpu = gpu_list.at(i)?;
        let chill = tds.get_chill(&gpu)?;
        let frtc = tds.get_frame_rate_target_control(&gpu)?;
        let vfr = tds.get_wait_for_vertical_refresh(&gpu)?;
        println!("  -- GPU #{i} --");
        println!(
            "  Chill: supported={} enabled={} fps_range={:?} min={} max={}",
            chill.is_supported()?,
            chill.is_enabled()?,
            chill.get_fps_range()?,
            chill.get_min_fps()?,
            chill.get_max_fps()?
        );
        println!(
            "  FRTC : supported={} enabled={} fps_range={:?} fps={}",
            frtc.is_supported()?,
            frtc.is_enabled()?,
            frtc.get_fps_range()?,
            frtc.get_fps()?
        );
        println!(
            "  VFR  : supported={} enabled={} mode={:?}",
            vfr.is_supported()?,
            vfr.is_enabled()?,
            vfr.get_mode()?
        );
    }

    Ok(())
}
