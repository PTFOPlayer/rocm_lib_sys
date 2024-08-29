use libloading::Symbol;

use crate::{error::RocmErr, RawRsmi};
pub const RSMI_MAX_NUM_FREQUENCIES: usize = 33;

impl RawRsmi {
    pub unsafe fn rsmi_dev_pci_bandwidth_get(
        &mut self,
        dv_ind: u32,
        bandwidth: *mut RsmiPcieBandwidth,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut RsmiPcieBandwidth) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_pci_bandwidth_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, bandwidth)
    }
    pub unsafe fn rsmi_dev_pci_id_get(&mut self, dv_ind: u32, id: *mut u64) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_pci_id_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, id)
    }
    pub unsafe fn rsmi_topo_numa_affinity_get(&mut self, dv_ind: u32, numa: *mut i32) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut i32) -> RocmErr> =
            match self.lib.get(b"rsmi_topo_numa_affinity_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, numa)
    }
    pub unsafe fn rsmi_dev_pci_throughput_get(
        &mut self,
        dv_ind: u32,
        sent: *mut u64,
        received: *mut u64,
        max_pkt_sz: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u64, *mut u64, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_topo_numa_affinity_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, sent, received, max_pkt_sz)
    }

    pub unsafe fn rsmi_dev_pci_replay_counter_get(
        &mut self,
        dv_ind: u32,
        counter: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_pci_replay_counter_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, counter)
    }
}

#[repr(C)]
pub struct RsmiPcieBandwidth {
    pub transfer_rate: RsmiFrequencies,
    pub lanes: [u32; RSMI_MAX_NUM_FREQUENCIES],
}

impl Default for RsmiPcieBandwidth {
    fn default() -> Self {
        Self {
            transfer_rate: Default::default(),
            lanes: [0u32; RSMI_MAX_NUM_FREQUENCIES],
        }
    }
}

#[repr(C)]
pub struct RsmiFrequencies {
    pub has_deep_sleep: bool,
    pub num_supported: u32,
    pub current: u32,
    pub frequency: [u64; RSMI_MAX_NUM_FREQUENCIES],
}
impl Default for RsmiFrequencies {
    fn default() -> Self {
        Self {
            has_deep_sleep: Default::default(),
            num_supported: Default::default(),
            current: Default::default(),
            frequency: [0u64; RSMI_MAX_NUM_FREQUENCIES],
        }
    }
}
