use libloading::Symbol;

use crate::{error::RocmErr, RawRsmi};

impl RawRsmi {
    pub unsafe fn rsmi_dev_fan_rpms_get(
        &mut self,
        dv_ind: u32,
        sensor: u32,
        rpm: *mut i64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, u32, *mut i64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_fan_rpms_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, sensor, rpm)
    }

    pub unsafe fn rsmi_dev_fan_speed_get(
        &mut self,
        dv_ind: u32,
        sensor: u32,
        speed: *mut i64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, u32, *mut i64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_fan_speed_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, sensor, speed)
    }

    pub unsafe fn rsmi_dev_fan_speed_max_get(
        &mut self,
        dv_ind: u32,
        sensor: u32,
        speed_max: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, u32, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_fan_speed_max_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, sensor, speed_max)
    }

    pub unsafe fn rsmi_dev_temp_metric_get(
        &mut self,
        dv_ind: u32,
        sensor: RsmiTemperatureSensor,
        metric: RsmiTemperatureMetric,
        temperature: *mut i64,
    ) -> RocmErr {
        let f: Symbol<
            unsafe extern "C" fn(
                u32,
                RsmiTemperatureSensor,
                RsmiTemperatureMetric,
                *mut i64,
            ) -> RocmErr,
        > = match self.lib.get(b"rsmi_dev_temp_metric_get") {
            Ok(res) => res,
            Err(err) => return err.into(),
        };
        f(dv_ind, sensor, metric, temperature)
    }

    pub unsafe fn rsmi_dev_volt_metric_get(
        &mut self,
        dv_ind: u32,
        voltage_type: RsmiVoltageType,
        metric: RsmiVoltageMetric,
        volt: *mut i64,
    ) -> RocmErr {
        let f: Symbol<
            unsafe extern "C" fn(u32, RsmiVoltageType, RsmiVoltageMetric, *mut i64) -> RocmErr,
        > = match self.lib.get(b"rsmi_dev_volt_metric_get") {
            Ok(res) => res,
            Err(err) => return err.into(),
        };
        f(dv_ind, voltage_type, metric, volt)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiVoltageMetric {
    RsmiVoltCurrent,
    RsmiVoltMax,
    RsmiVoltMinCrit,
    RsmiVoltMin,
    RsmiVoltMaxCrit,
    RsmiVoltAverage,
    RsmiVoltLowest,
    RsmiVoltHighest,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiTemperatureMetric {
    RsmiTempCurrent,
    RsmiTempMax,
    RsmiTempMin,
    RsmiTempMaxHyst,
    RsmiTempMinHyst,
    RsmiTempCritical,
    RsmiTempCriticalHyst,
    RsmiTempEmergency,
    RsmiTempEmergencyHyst,
    RsmiTempCritMin,
    RsmiTempCritMinHyst,
    RsmiTempOffset,
    RsmiTempLowest,
    RsmiTempHighest,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiTemperatureSensor {
    RsmiTempTypeEdge,
    RsmiTempTypeJunction,
    RsmiTempTypeMemory,
    RsmiTempTypeHbm0,
    RsmiTempTypeHbm1,
    RsmiTempTypeHbm2,
    RsmiTempTypeHbm3,
    RsmiTempTypeInvalid,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub enum RsmiVoltageType {
    RsmiVoltTypeVddgfx = 0,
    #[default]
    RsmiVoltTypeInvalid = 0xFFFFFFFF,
}
