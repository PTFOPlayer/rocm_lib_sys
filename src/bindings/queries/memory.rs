use crate::error::RocmErr;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    // memory
    pub fn rsmi_dev_memory_total_get(
        dv_ind: u32,
        mem_type: RsmiMemoryType,
        total: *mut u64,
    ) -> RocmErr;
    pub fn rsmi_dev_memory_usage_get(
        dv_ind: u32,
        mem_type: RsmiMemoryType,
        usage: *mut u64,
    ) -> RocmErr;
    pub fn rsmi_dev_memory_busy_percent_get(dv_ind: u32, percent: *mut u32) -> RocmErr;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiMemoryType {
    RsmiMemTypeVram,
    RsmiMemTypeVisVram,
    RsmiMemTypeGtt,
}
