//! 交叉扫描：找出每个 ADLX 接口的真实父对象
//!
//! 对每个可达对象（System / GPU / Metrics / Display ...）逐一尝试 QueryInterface，
//! 从而确定「谁支持哪些接口」——这正是 1.5 新增接口能否被项目用上的关键。

use adlx_lite::ffi;
use adlx_lite::helper::AdlxHelper;
use adlx_lite::interface::{Interface, InterfaceImpl};

/// 扫描一个父对象，把它支持的 IID 收集起来
macro_rules! scan {
    ($parent:expr, $label:literal, [$($iid:literal),* $(,)?]) => {{
        let mut found: Vec<&str> = Vec::new();
        $(
            {
                #[repr(transparent)]
                struct P(InterfaceImpl);
                unsafe impl Interface for P {
                    type Impl = ffi::IADLXInterface;
                    type Vtable = ffi::IADLXInterfaceVtbl;
                    const IID: &'static str = $iid;
                }
                if $parent.cast::<P>().is_ok() {
                    found.push($iid);
                }
            }
        )*
        if !found.is_empty() {
            println!("  {:<32} ← {}", $label, found.join(", "));
        }
    }};
}

/// 用固定候选集扫描一个父对象
macro_rules! scan_all {
    ($parent:expr, $label:literal) => {
        scan!(
            $parent,
            $label,
            [
                // 1.4 已有的版本化接口（对照组，验证扫描方法有效）
                "IADLXSystem1",
                "IADLXSystem2",
                "IADLXGPU1",
                "IADLXGPU2",
                "IADLXDisplayServices1",
                "IADLXDisplayServices2",
                "IADLXDisplayServices3",
                "IADLXGPUMetrics1",
                "IADLXGPUMetricsSupport1",
                // 1.5 新增
                "IADLXSystem3",
                "IADLXGPU3",
                "IADLXGPUMetrics2",
                "IADLXGPUMetrics3",
                "IADLXGPUMetricsSupport2",
                "IADLXGPUMetricsSupport3",
                "IADLXDisplayServicesPartner_V1",
                "IADLXVariableGraphicsMemory",
                "IADLXVariableGraphicsMemoryOption",
                "IADLXVariableGraphicsMemoryOptionList",
                "IADLXDisplayCVDC",
                "IADLXDisplayAllColorFeaturesPartner_V1",
                "IADLXDisplayPanelFirmwareUpdatePartner_V1",
            ]
        )
    };
}

fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("ADLX_DLL_PATH") {
        let _lib = Box::leak(Box::new(unsafe { libloading::Library::new(&path)? }));
        println!("预加载 DLL: {path}");
    }

    let helper = AdlxHelper::new()?;
    println!("ADLX 运行时版本: {}\n", helper.version());
    let system = helper.system();

    // ---- 收集所有可达对象 ----
    let gpu = system.gpus()?.at(0)?;
    let ds = system.get_displays_services()?;
    let display = ds.get_displays()?.at(0)?;
    let pms = system.performance_monitoring_services()?;
    let metrics = pms.current_gpu_metrics(&gpu)?;
    let support = pms.supported_gpu_metrics(&gpu)?;
    let tds = system.get_3d_settings_services()?;
    let chill = tds.get_chill(&gpu)?;
    let frtc = tds.get_frame_rate_target_control(&gpu)?;
    let wfv = tds.get_wait_for_vertical_refresh(&gpu)?;
    let fs = ds.get_free_sync(&display)?;

    println!("=== 各父对象支持的接口 ===");
    scan_all!(system, "IADLXSystem");
    scan_all!(gpu, "IADLXGPU");
    scan_all!(metrics, "IADLXGPUMetrics");
    scan_all!(support, "IADLXGPUMetricsSupport");
    scan_all!(pms, "IADLXPerformanceMonitoringSvc");
    scan_all!(ds, "IADLXDisplayServices");
    scan_all!(display, "IADLXDisplay");
    scan_all!(tds, "IADLX3DSettingsServices");
    scan_all!(chill, "IADLX3DChill");
    scan_all!(frtc, "IADLX3DFrameRateTargetCtrl");
    scan_all!(wfv, "IADLX3DWaitForVerticalRefresh");
    scan_all!(fs, "IADLXFreeSync");

    Ok(())
}
