use crate::error::RocmErr;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    //physical
    pub fn rsmi_dev_fan_rpms_get(dv_ind: u32, sensor: u32, rpm: *mut i64) -> RocmErr;
    pub fn rsmi_dev_fan_speed_get(dv_ind: u32, sensor: u32, speed: *mut i64) -> RocmErr;
    pub fn rsmi_dev_fan_speed_max_get(dv_ind: u32, sensor: u32, speed_max: *mut u64) -> RocmErr;
    pub fn rsmi_dev_temp_metric_get(
        dv_ind: u32,
        sensor: RsmiTemperatureSensor,
        metric: RsmiTemperatureMetric,
        temperature: *mut i64,
    ) -> RocmErr;
    pub fn rsmi_dev_volt_metric_get(
        dv_ind: u32,
        voltage_type: RsmiVoltageTypeT,
        metric: RsmiVoltageMetric,
        volt: *mut i64,
    ) -> RocmErr;
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
pub enum RsmiVoltageTypeT {
    RsmiVoltTypeVddgfx = 0,
    #[default]
    RsmiVoltTypeInvalid = 0xFFFFFFFF,
}
