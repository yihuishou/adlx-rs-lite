//! GPU 压力测试完成监听器。
//!
//! ADLX 的压力测试是异步的：调用 [`crate::gpu::Gpu3::start_stress_test`] 之后，
//! ADLX 在自己的线程上回调监听器通知测试结束。
//!
//! 这类监听器不是标准的 COM 接口（没有 `QueryInterface`/`AddRef`/`Release`），
//! vtable 里只有一个 `OnGPUStressTestFinished` 方法，因此这里的实现只需构造一张
//! 静态函数表，并把回调指针藏在对象首字段之后。

use super::ffi;

/// 压力测试结束时的回调。
///
/// 参数是测试是否成功。返回 `true` 表示继续通知其它监听器，返回 `false` 表示停止通知。
pub type StressTestCallback = Box<dyn FnMut(bool) -> bool + Send>;

#[repr(C)]
struct ListenerImpl {
    p_vtbl: *const ffi::IADLXGPUStressTestFinishedListenerVtbl,
    callback: StressTestCallback,
}

static VTABLE: ffi::IADLXGPUStressTestFinishedListenerVtbl =
    ffi::IADLXGPUStressTestFinishedListenerVtbl {
        OnGPUStressTestFinished: Some(on_gpu_stress_test_finished),
    };

unsafe extern "C" fn on_gpu_stress_test_finished(
    p_this: *mut ffi::IADLXGPUStressTestFinishedListener,
    _p_gpu: *mut ffi::IADLXGPU3,
    result: ffi::adlx_bool,
) -> ffi::adlx_bool {
    let listener = p_this as *mut ListenerImpl;
    let callback = &mut (*listener).callback;
    callback(result != 0) as ffi::adlx_bool
}

/// GPU 压力测试完成监听器。
///
/// # 生命周期
///
/// ADLX 持有的是指向本对象的裸指针，而且**回调触发之后仍会继续访问它**
/// （实测在 `ADLXTerminate` 收尾阶段依然会访问）。
/// 因此本监听器一旦创建就**不会被释放**——这是有意为之，
/// 否则 ADLX 访问已释放内存会导致进程以 `0xc0000409` 退出。
/// 泄漏的量是每个监听器几十字节，进程结束时由操作系统回收。
///
/// # 例子
///
/// ```no_run
/// use adlx::{Gpu3, GpuStressTestFinishedListener};
///
/// # fn run(gpu3: &Gpu3) -> adlx::Result<()> {
/// let mut listener = GpuStressTestFinishedListener::new(|success| {
///     println!("压力测试结束，成功: {success}");
///     true
/// });
/// gpu3.start_stress_test(&mut listener, 5)?;
/// # Ok(())
/// # }
/// ```
pub struct GpuStressTestFinishedListener {
    inner: &'static mut ListenerImpl,
}

// ListenerImpl 内部只含一个 `Send` 的回调，且 `&mut` 独占保证不会并发调用回调。
unsafe impl Send for GpuStressTestFinishedListener {}

impl GpuStressTestFinishedListener {
    /// 创建监听器，`callback` 会在压力测试结束时由 ADLX 调用。
    pub fn new<F>(callback: F) -> Self
    where
        F: FnMut(bool) -> bool + Send + 'static,
    {
        Self {
            inner: Box::leak(Box::new(ListenerImpl {
                p_vtbl: &VTABLE,
                callback: Box::new(callback),
            })),
        }
    }

    pub(crate) fn as_raw(&mut self) -> *mut ffi::IADLXGPUStressTestFinishedListener {
        self.inner as *mut ListenerImpl as *mut ffi::IADLXGPUStressTestFinishedListener
    }
}

impl std::fmt::Debug for GpuStressTestFinishedListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GpuStressTestFinishedListener").finish()
    }
}
