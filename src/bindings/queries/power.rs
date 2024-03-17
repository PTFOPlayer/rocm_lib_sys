use libloading::Symbol;

use crate::{error::RocmErr, RawRsmi};

impl RawRsmi {
    pub unsafe fn rsmi_dev_power_ave_get(
        &mut self,
        dv_ind: u32,
        sensor: u32,
        ave: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, u32, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_power_ave_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, sensor, ave)
    }

    pub unsafe fn rsmi_dev_current_socket_power_get(
        &mut self,
        dv_ind: u32,
        power: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_current_socket_power_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, power)
    }

    pub unsafe fn rsmi_dev_power_get(
        &mut self,
        dv_ind: u32,
        power: *mut u64,
        pwr_type: *mut RsmiPowerType,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u64, *mut RsmiPowerType) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_power_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, power, pwr_type)
    }

    pub unsafe fn rsmi_dev_energy_count_get(
        &mut self,
        dv_ind: u32,
        power: *mut u64,
        counter_resolution: *mut f32,
        time_stamp: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u64, *mut f32, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_energy_count_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, power, counter_resolution, time_stamp)
    }

    pub unsafe fn rsmi_dev_power_cap_get(
        &mut self,
        dv_ind: u32,
        sensor: u32,
        cap: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, u32, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_power_cap_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, sensor, cap)
    }

    pub unsafe fn rsmi_dev_power_cap_range_get(
        &mut self,
        dv_ind: u32,
        sensor: u32,
        max: *mut u64,
        min: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, u32, *mut u64, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_power_cap_range_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, sensor, max, min)
    }

    pub unsafe fn rsmi_dev_power_cap_default_get(
        &mut self,
        dv_ind: u32,
        default: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_power_cap_default_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, default)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiPowerType {
    RsmiAveragePower = 0,
    RsmiCurrentPower,
    RsmiInvalidPower = 0xFFFFFFFF,
}
