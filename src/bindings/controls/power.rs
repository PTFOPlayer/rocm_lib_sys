use libloading::Symbol;

use crate::{bindings::RsmiPowerProfilePresetMasks, error::RocmErr, RawRsmi};

impl RawRsmi {
    pub unsafe fn rsmi_dev_power_cap_set(&mut self, dv_ind: u32, sensor: u32, cap: u64) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, u32, u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_power_cap_set") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, sensor, cap)
    }

    // reserved not currently used, set to 0
    pub unsafe fn rsmi_dev_power_profile_set(
        &mut self,
        dv_ind: u32,
        reserved: u32,
        profile: RsmiPowerProfilePresetMasks,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, u32, RsmiPowerProfilePresetMasks) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_power_profile_set") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, reserved, profile)
    }

    pub unsafe fn rsmi_dev_power_profile_set_v0(
        &mut self,
        dv_ind: u32,
        profile: RsmiPowerProfilePresetMasks,
    ) -> RocmErr {
        self.rsmi_dev_power_profile_set(dv_ind, 0, profile)
    }
}
