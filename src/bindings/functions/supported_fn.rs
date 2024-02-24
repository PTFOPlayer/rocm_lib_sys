use crate::{error::RocmErr, bindings::{RsmiMemoryType, RsmiTemperatureMetric, RsmiClkType, RsmiFwBlock, RsmiGpuBlock}};

use super::perf_counter::{RsmiEventType, RsmiEventGroup};

#[link(name = "rsmi64", kind = "static")]
extern "C" {

    pub fn rsmi_dev_supported_func_iterator_open(
        dv_ind: u32,
        handle: *mut RsmiFuncIdIterHandle,
    ) -> RocmErr;
    pub fn rsmi_func_iter_next(handle: RsmiFuncIdIterHandle) -> RocmErr;
    pub fn rsmi_func_iter_value_get(
        handle: RsmiFuncIdIterHandle,
        value: *mut RsmiFuncIdValue,
    ) -> RocmErr;
    pub fn rsmi_dev_supported_func_iterator_close(handle: *mut RsmiFuncIdIterHandle) -> RocmErr;

    pub fn 	rsmi_dev_supported_variant_iterator_open (
        handle: RsmiFuncIdIterHandle,
        var_iter: *mut RsmiFuncIdIterHandle,
    ) -> RocmErr;
}

pub const RSMI_DEFAULT_VARIANT: u64 = 0xFFFFFFFFFFFFFFFF; 

#[allow(conflicting_repr_hints)]
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RsmiFuncIdIterHandle {_f:u64}

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
        Self{id: 0}
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
