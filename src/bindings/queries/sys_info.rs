use libloading::Symbol;

use crate::{error::RocmErr, RawRsmi};

impl RawRsmi {
    pub unsafe fn rsmi_compute_process_info_get(
        &mut self,
        procs: *mut RsmiProcessInfo,
        num_items: *mut u32,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(*mut RsmiProcessInfo, *mut u32) -> RocmErr> =
            match self.lib.get(b"rsmi_compute_process_info_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(procs, num_items)
    }
    pub unsafe fn rsmi_compute_process_info_by_pid_get(
        &mut self,
        pid: u32,
        proc: *mut RsmiProcessInfo,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut RsmiProcessInfo) -> RocmErr> =
            match self.lib.get(b"rsmi_compute_process_info_by_pid_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(pid, proc)
    }
    pub unsafe fn rsmi_compute_process_gpus_get(
        &mut self,
        pid: u32,
        dv_indices: *mut u32,
        num_devices: *mut u32,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u32, *mut u32) -> RocmErr> =
            match self.lib.get(b"rsmi_compute_process_gpus_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(pid, dv_indices, num_devices)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RsmiProcessInfo {
    pub process_id: u32,
    pub pasid: u32,
    pub vram_usage: u64,
    pub sdma_usage: u64,
    pub cu_occupancy: u32,
}
