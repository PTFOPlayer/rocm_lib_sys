use libloading::Symbol;

use crate::{error::RocmErr, RawRsmi};

impl RawRsmi {
    pub unsafe fn rsmi_dev_memory_total_get(
        &mut self,
        dv_ind: u32,
        mem_type: RsmiMemoryType,
        total: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, RsmiMemoryType, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_memory_total_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, mem_type, total)
    }
    pub unsafe fn rsmi_dev_memory_usage_get(
        &mut self,
        dv_ind: u32,
        mem_type: RsmiMemoryType,
        usage: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, RsmiMemoryType, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_memory_usage_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, mem_type, usage)
    }
    pub unsafe fn rsmi_dev_memory_busy_percent_get(
        &mut self,
        dv_ind: u32,
        percent: *mut u32,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u32) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_memory_busy_percent_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, percent)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiMemoryType {
    RsmiMemTypeVram,
    RsmiMemTypeVisVram,
    RsmiMemTypeGtt,
}
