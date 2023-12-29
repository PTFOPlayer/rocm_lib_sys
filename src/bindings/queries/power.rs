use crate::error::RocmErr;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    pub fn rsmi_dev_power_ave_get(dv_ind: u32, sensor: u32, ave: *mut u64) -> RocmErr;
    pub fn rsmi_dev_power_cap_get(dv_ind: u32, sensor: u32, cap: *mut u64) -> RocmErr;
    pub fn rsmi_dev_power_cap_range_get(
        dv_ind: u32,
        sensor: u32,
        max: *mut u64,
        min: *mut u64,
    ) -> RocmErr;
    pub fn rsmi_dev_power_cap_default_get(dv_ind: u32, default: *mut u64) -> RocmErr;
}
