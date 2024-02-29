use crate::{
    bindings::{RsmiClkType, RsmiFwBlock, RsmiGpuBlock, RsmiMemoryType, RsmiTemperatureMetric},
    error::RocmErr,
    function_creator, RawRsmi,
};

use super::perf_counter::{RsmiEventGroup, RsmiEventType};

impl RawRsmi {
    pub unsafe fn rsmi_dev_supported_func_iterator_open(
        &mut self,
        dv_ind: u32,
        handle: *mut RsmiFuncIdIterHandle,
    ) -> RocmErr {
        function_creator!(self, b"rsmi_dev_supported_func_iterator_open", <u32, *mut RsmiFuncIdIterHandle>, (dv_ind, handle))
    }
    pub unsafe fn rsmi_func_iter_next(&mut self, handle: RsmiFuncIdIterHandle) -> RocmErr {
        function_creator!(self, b"rsmi_func_iter_next", <RsmiFuncIdIterHandle>, (handle))
    }
    pub unsafe fn rsmi_func_iter_value_get(
        &mut self,
        handle: RsmiFuncIdIterHandle,
        value: *mut RsmiFuncIdValue,
    ) -> RocmErr {
        function_creator!(self, b"rsmi_func_iter_next", <RsmiFuncIdIterHandle, *mut RsmiFuncIdValue>, (handle, value))
    }
    pub unsafe fn rsmi_dev_supported_func_iterator_close(
        &mut self,
        handle: *mut RsmiFuncIdIterHandle,
    ) -> RocmErr {
        function_creator!(self, b"rsmi_dev_supported_func_iterator_close", <*mut RsmiFuncIdIterHandle>, (handle))
    }

    pub unsafe fn rsmi_dev_supported_variant_iterator_open(
        &mut self,
        handle: RsmiFuncIdIterHandle,
        var_iter: *mut RsmiFuncIdIterHandle,
    ) -> RocmErr {
        function_creator!(self, b"rsmi_dev_supported_variant_iterator_open", <RsmiFuncIdIterHandle, *mut RsmiFuncIdIterHandle>, (handle, var_iter))
    }
}

pub const RSMI_DEFAULT_VARIANT: u64 = 0xFFFFFFFFFFFFFFFF;

#[allow(conflicting_repr_hints)]
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RsmiFuncIdIterHandle {
    _f: u64,
}

impl RsmiFuncIdIterHandle {
    pub fn new() -> Self {
        Self::default()
    }
}

#[repr(C)]
pub union RsmiFuncIdValue {
    pub id: u64,
    pub name: *mut u8,
    pub inner: RsmiFuncIdValueInner,
}

impl Default for RsmiFuncIdValue {
    fn default() -> Self {
        Self { id: 0 }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RsmiFuncIdValueInner {
    pub memory_type: RsmiMemoryType,
    pub temp_metric: RsmiTemperatureMetric,
    pub evnt_type: RsmiEventType,
    pub evnt_group: RsmiEventGroup,
    pub clk_type: RsmiClkType,
    pub fw_block: RsmiFwBlock,
    pub gpu_block_type: RsmiGpuBlock,
}
