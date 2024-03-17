use libloading::Symbol;

use crate::{error::RocmErr, RawRsmi};

impl RawRsmi {
    pub unsafe fn rsmi_dev_pci_bandwidth_set(&mut self, dv_ind: u32, bw_bitmask: u64) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_pci_bandwidth_set") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, bw_bitmask)
    }
}
