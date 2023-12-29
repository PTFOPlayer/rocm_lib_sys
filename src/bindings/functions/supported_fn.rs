use crate::{error::RocmErr, bindings::{RsmiMemoryType, RsmiTemperatureMetric, RsmiClkType, RsmiFwBlockT, RsmiGpuBlockT}};

use super::perf_counter::{RsmiEventTypeT, RsmiEventGroupT};

#[link(name = "rsmi64", kind = "static")]
extern "C" {

    pub fn rsmi_dev_supported_func_iterator_open(
        dv_ind: u32,
        handle: *mut RsmiFuncIdIterHandleT,
    ) -> RocmErr;
    pub fn rsmi_func_iter_next(handle: RsmiFuncIdIterHandleT) -> RocmErr;
    pub fn rsmi_func_iter_value_get(
        handle: RsmiFuncIdIterHandleT,
        value: *mut RsmiFuncIdValueT,
    ) -> RocmErr;
    pub fn rsmi_dev_supported_func_iterator_close(handle: *mut RsmiFuncIdIterHandleT) -> RocmErr;

    pub fn 	rsmi_dev_supported_variant_iterator_open (
        handle: RsmiFuncIdIterHandleT,
        var_iter: *mut RsmiFuncIdIterHandleT,
    ) -> RocmErr;
}

pub const RSMI_DEFAULT_VARIANT: u64 = 0xFFFFFFFFFFFFFFFF; 

#[allow(conflicting_repr_hints)]
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RsmiFuncIdIterHandleT {_f:u64}

impl RsmiFuncIdIterHandleT {
    pub fn new() -> Self {
        Self::default()
    }
}

#[repr(C)]
pub union RsmiFuncIdValueT {
    pub id: u64,
    pub name: *mut u8,
    pub inner: RsmiFuncIdValueTInner,
}

impl Default for RsmiFuncIdValueT {
    fn default() -> Self {
        Self{id: 0}
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RsmiFuncIdValueTInner {
    pub memory_type: RsmiMemoryType,
    pub temp_metric: RsmiTemperatureMetric,
    pub evnt_type: RsmiEventTypeT,
    pub evnt_group: RsmiEventGroupT,
    pub clk_type: RsmiClkType,
    pub fw_block: RsmiFwBlockT,
    pub gpu_block_type: RsmiGpuBlockT,
}
