use crate::error::RocmErr;
pub const RSMI_MAX_NUM_FREQUENCIES: usize = 32;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    pub fn rsmi_dev_pci_bandwidth_get(dv_ind: u32, bandwidth: *mut RsmiPcieBandwidth) -> RocmErr;
    pub fn rsmi_dev_pci_id_get(dv_ind: u32, id: *mut u64) -> RocmErr;
    pub fn rsmi_topo_numa_affinity_get(dv_ind: u32, numa: *mut u32) -> RocmErr;
    pub fn rsmi_dev_pci_throughput_get(
        dv_ind: u32,
        sent: *mut u64,
        received: *mut u64,
        max_pkt_sz: *mut u64,
    ) -> RocmErr;
}



#[repr(C)]
#[derive(Default)]
pub struct RsmiPcieBandwidth {
    pub transfer_rate: RsmiFrequencies,
    pub lanes: [u32; RSMI_MAX_NUM_FREQUENCIES],
}

#[repr(C)]
#[derive(Default)]
pub struct RsmiFrequencies {
    pub num_supported: u32,
    pub current: u32,
    pub frequency: [u64; RSMI_MAX_NUM_FREQUENCIES],
}
