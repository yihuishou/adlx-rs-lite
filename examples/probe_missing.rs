//! 穷举父对象，寻找 4 个「漏网」接口的真实入口

use adlx_lite::ffi;
use adlx_lite::helper::AdlxHelper;
use adlx_lite::interface::{Interface, InterfaceImpl};

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
            println!("  {:<34} ← {}", $label, found.join(", "));
        }
    }};
}

/// 造一个只有 IID 的最小接口类型并 cast 出来
macro_rules! probe_type {
    ($parent:expr, $iid:literal) => {{
        #[repr(transparent)]
        struct P(InterfaceImpl);
        unsafe impl Interface for P {
            type Impl = ffi::IADLXInterface;
            type Vtable = ffi::IADLXInterfaceVtbl;
            const IID: &'static str = $iid;
        }
        $parent.cast::<P>().ok()
    }};
}

fn main() -> anyhow::Result<()> {
    if let Ok(path) = std::env::var("ADLX_DLL_PATH") {
        let _lib = Box::leak(Box::new(unsafe { libloading::Library::new(&path)? }));
        println!("预加载 DLL: {path}");
    }

    let helper = AdlxHelper::new()?;
    println!("ADLX 运行时版本: {}\n", helper.version());
    let system = helper.system();

    let gpu = system.gpus()?.at(0)?;
    let ds = system.get_displays_services()?;
    let display = ds.get_displays()?.at(0)?;
    let pms = system.performance_monitoring_services()?;
    let metrics = pms.current_gpu_metrics(&gpu)?;
    let support = pms.supported_gpu_metrics(&gpu)?;
    let tds = system.get_3d_settings_services()?;

    let sys1 = probe_type!(system, "IADLXSystem1");
    let sys2 = probe_type!(system, "IADLXSystem2");
    let sys3 = probe_type!(system, "IADLXSystem3");
    let gpu1 = probe_type!(gpu, "IADLXGPU1");
    let gpu2 = probe_type!(gpu, "IADLXGPU2");
    let gpu3 = probe_type!(gpu, "IADLXGPU3");
    let ds1 = probe_type!(ds, "IADLXDisplayServices1");
    let ds2 = probe_type!(ds, "IADLXDisplayServices2");
    let ds3 = probe_type!(ds, "IADLXDisplayServices3");
    let ds_partner = probe_type!(ds, "IADLXDisplayServicesPartner_V1");
    let m1 = probe_type!(metrics, "IADLXGPUMetrics1");
    let m2 = probe_type!(metrics, "IADLXGPUMetrics2");
    let m3 = probe_type!(metrics, "IADLXGPUMetrics3");
    let s1 = probe_type!(support, "IADLXGPUMetricsSupport1");
    let s2 = probe_type!(support, "IADLXGPUMetricsSupport2");
    let s3 = probe_type!(support, "IADLXGPUMetricsSupport3");

    macro_rules! scan_all {
        ($parent:expr, $label:literal) => {
            scan!(
                $parent,
                $label,
                [
                    "IADLXSystem3",
                    "IADLXGPU3",
                    "IADLXGPUMetrics3",
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
    println!("=== 穷举父对象 ===");
    scan_all!(system, "IADLXSystem");
    scan_all!(sys1.as_ref().unwrap(), "IADLXSystem1");
    scan_all!(sys2.as_ref().unwrap(), "IADLXSystem2");
    scan_all!(sys3.as_ref().unwrap(), "IADLXSystem3");
    scan_all!(gpu, "IADLXGPU");
    scan_all!(gpu1.as_ref().unwrap(), "IADLXGPU1");
    scan_all!(gpu2.as_ref().unwrap(), "IADLXGPU2");
    scan_all!(gpu3.as_ref().unwrap(), "IADLXGPU3");
    scan_all!(ds, "IADLXDisplayServices");
    scan_all!(ds1.as_ref().unwrap(), "IADLXDisplayServices1");
    scan_all!(ds2.as_ref().unwrap(), "IADLXDisplayServices2");
    scan_all!(ds3.as_ref().unwrap(), "IADLXDisplayServices3");
    scan_all!(
        ds_partner.as_ref().unwrap(),
        "IADLXDisplayServicesPartner_V1"
    );
    scan_all!(display, "IADLXDisplay");
    scan_all!(metrics, "IADLXGPUMetrics");
    scan_all!(m1.as_ref().unwrap(), "IADLXGPUMetrics1");
    scan_all!(m2.as_ref().unwrap(), "IADLXGPUMetrics2");
    scan_all!(m3.as_ref().unwrap(), "IADLXGPUMetrics3");
    scan_all!(support, "IADLXGPUMetricsSupport");
    scan_all!(s1.as_ref().unwrap(), "IADLXGPUMetricsSupport1");
    scan_all!(s2.as_ref().unwrap(), "IADLXGPUMetricsSupport2");
    scan_all!(s3.as_ref().unwrap(), "IADLXGPUMetricsSupport3");
    scan_all!(pms, "IADLXPerformanceMonitoringSvc");
    scan_all!(tds, "IADLX3DSettingsServices");

    Ok(())
}
