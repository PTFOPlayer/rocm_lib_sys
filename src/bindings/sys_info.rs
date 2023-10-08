use crate::error::RocmErr;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    pub fn rsmi_compute_process_info_get(procs: * mut RsmiProcessInfoT, num_items: *mut u32) -> RocmErr;
    pub fn 	rsmi_compute_process_info_by_pid_get(pid: u32, proc: *mut RsmiProcessInfoT) -> RocmErr;
    pub fn 	rsmi_compute_process_gpus_get(pid: u32, dv_indices: *mut u32, num_devices: *mut u32) -> RocmErr;
}


#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RsmiProcessInfoT {
    pub process_id: u32,      
    pub pasid: u32,           
    pub vram_usage: u64,      
    pub sdma_usage: u64,      
    pub cu_occupancy: u32    
}