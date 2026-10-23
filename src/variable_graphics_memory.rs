//! ADLX 1.5 新增的可变速显存（Variable Graphics Memory，VGM）接口。
//!
//! 可变速显存允许把系统内存的一部分切分给显卡当作显存使用。
//! 通过 [`crate::system::System3::variable_graphics_memory()`] 获取 [`VariableGraphicsMemory`]
//! 实例，再用 [`VariableGraphicsMemory::is_supported()`] 确认硬件是否支持。

use std::{ffi::CStr, mem::MaybeUninit};

use super::{
    ffi,
    interface::{Interface, InterfaceImpl},
    result::{Error, Result},
};

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_variable_graphics_memory/>
///
/// 可变速显存配置接口。
#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXVariableGraphicsMemory")]
pub struct VariableGraphicsMemory(InterfaceImpl);

unsafe impl Interface for VariableGraphicsMemory {
    type Impl = ffi::IADLXVariableGraphicsMemory;
    type Vtable = ffi::IADLXVariableGraphicsMemoryVtbl;
    const IID: &'static str = "IADLXVariableGraphicsMemory";
}

impl VariableGraphicsMemory {
    /// 当前显卡是否支持可变速显存。
    #[doc(alias = "IsSupported")]
    pub fn is_supported(&self) -> Result<bool> {
        let mut supported = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().IsSupported.unwrap())(self.as_raw(), supported.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, supported).map(|x| x != 0)
    }

    /// 当前生效的显存配置。
    #[doc(alias = "GetOption")]
    pub fn option(&self) -> Result<VariableGraphicsMemoryOption> {
        let mut option = MaybeUninit::uninit();
        let result =
            unsafe { (self.vtable().GetOption.unwrap())(self.as_raw(), option.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, option)
            .map(|option| unsafe { VariableGraphicsMemoryOption::from_raw(option) })
    }

    /// 出厂默认的显存配置。
    #[doc(alias = "GetDefaultOption")]
    pub fn default_option(&self) -> Result<VariableGraphicsMemoryOption> {
        let mut option = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetDefaultOption.unwrap())(self.as_raw(), option.as_mut_ptr())
        };

        Error::from_result_with_assume_init_on_success(result, option)
            .map(|option| unsafe { VariableGraphicsMemoryOption::from_raw(option) })
    }

    /// 所有可选的显存配置。
    #[doc(alias = "GetAvailableOptions")]
    pub fn available_options(&self) -> Result<VariableGraphicsMemoryOptionList> {
        let mut options = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().GetAvailableOptions.unwrap())(self.as_raw(), options.as_mut_ptr())
        };

        Error::from_result_with_assume_init_on_success(result, options)
            .map(|options| unsafe { VariableGraphicsMemoryOptionList::from_raw(options) })
    }

    /// 应用一个显存配置。
    #[doc(alias = "SetOption")]
    pub fn set_option(&self, option: &VariableGraphicsMemoryOption) -> Result<()> {
        let result = unsafe { (self.vtable().SetOption.unwrap())(self.as_raw(), option.as_raw()) };

        Error::from_result(result)
    }
}

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_variable_graphics_memory_option/>
///
/// 一份可变速显存配置。
#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXVariableGraphicsMemoryOption")]
pub struct VariableGraphicsMemoryOption(InterfaceImpl);

unsafe impl Interface for VariableGraphicsMemoryOption {
    type Impl = ffi::IADLXVariableGraphicsMemoryOption;
    type Vtable = ffi::IADLXVariableGraphicsMemoryOptionVtbl;
    const IID: &'static str = "IADLXVariableGraphicsMemoryOption";
}

impl VariableGraphicsMemoryOption {
    /// 配置的名称。
    #[doc(alias = "Name")]
    pub fn name(&self) -> Result<&str> {
        let mut name = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().Name.unwrap())(self.as_raw(), name.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, name)
            .map(|name| unsafe { CStr::from_ptr(name) }.to_str().unwrap())
    }

    /// 配置的模式：自动或自定义。
    #[doc(alias = "Mode")]
    pub fn mode(&self) -> Result<ffi::ADLX_VARIABLE_GRAPHICS_MEMORY_MODE> {
        let mut mode = MaybeUninit::uninit();
        let result = unsafe { (self.vtable().Mode.unwrap())(self.as_raw(), mode.as_mut_ptr()) };

        Error::from_result_with_assume_init_on_success(result, mode)
    }

    /// 划给显存的系统内存大小，单位 GB。
    #[doc(alias = "MemoryCarved")]
    pub fn memory_carved(&self) -> Result<f64> {
        let mut memory_carved = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().MemoryCarved.unwrap())(self.as_raw(), memory_carved.as_mut_ptr())
        };

        Error::from_result_with_assume_init_on_success(result, memory_carved)
    }

    /// 未划给显存的系统内存大小，单位 GB。
    #[doc(alias = "MemoryRemaining")]
    pub fn memory_remaining(&self) -> Result<f64> {
        let mut memory_remaining = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().MemoryRemaining.unwrap())(self.as_raw(), memory_remaining.as_mut_ptr())
        };

        Error::from_result_with_assume_init_on_success(result, memory_remaining)
    }
}

/// <https://gpuopen.com/manuals/adlx/adlx-_d_o_x__i_a_d_l_x_variable_graphics_memory_option_list/>
///
/// 可变速显存配置的列表。
#[derive(Clone, Debug)]
#[repr(transparent)]
#[doc(alias = "IADLXVariableGraphicsMemoryOptionList")]
pub struct VariableGraphicsMemoryOptionList(InterfaceImpl);

unsafe impl Interface for VariableGraphicsMemoryOptionList {
    type Impl = ffi::IADLXVariableGraphicsMemoryOptionList;
    type Vtable = ffi::IADLXVariableGraphicsMemoryOptionListVtbl;
    const IID: &'static str = "IADLXVariableGraphicsMemoryOptionList";
}

impl VariableGraphicsMemoryOptionList {
    /// 列表中的配置数量。
    #[doc(alias = "Size")]
    pub fn size(&self) -> Result<u32> {
        Ok(unsafe { (self.vtable().Size.unwrap())(self.as_raw()) })
    }

    /// 取得列表中第 `location` 项配置。
    #[doc(alias = "At_OptionList")]
    pub fn at(&self, location: u32) -> Result<VariableGraphicsMemoryOption> {
        let mut option = MaybeUninit::uninit();
        let result = unsafe {
            (self.vtable().At_OptionList.unwrap())(self.as_raw(), location, option.as_mut_ptr())
        };

        Error::from_result_with_assume_init_on_success(result, option)
            .map(|option| unsafe { VariableGraphicsMemoryOption::from_raw(option) })
    }
}
