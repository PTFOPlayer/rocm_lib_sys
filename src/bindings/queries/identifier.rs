use crate::error::RocmErr;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    pub fn rsmi_num_monitor_devices(num_devices: *mut u32) -> RocmErr;
    pub fn rsmi_dev_id_get(dv_ind: u32, id: *mut u16) -> RocmErr;
    pub fn rsmi_dev_name_get(dv_ind: u32, name: *mut i8, name_length: usize) -> RocmErr;
    pub fn rsmi_dev_vendor_id_get(dv_ind: u32, id: *mut u16) -> RocmErr;
    pub fn rsmi_dev_brand_get(dv_ind: u32, brand: *mut i8, name_length: usize) -> RocmErr;
    pub fn rsmi_dev_vendor_name_get(dv_ind: u32, vendor: *mut i8, name_length: usize) -> RocmErr;
    pub fn rsmi_dev_vram_vendor_get(dv_ind: u32, vendor: *mut i8, name_length: usize) -> RocmErr;
    pub fn rsmi_dev_serial_number_get(
        dv_ind: u32,
        serial_number: *mut i8,
        name_length: usize,
    ) -> RocmErr;
    pub fn rsmi_dev_subsystem_id_get(dv_ind: u32, id: *mut u16) -> RocmErr;
    pub fn rsmi_dev_subsystem_name_get(
        dv_ind: u32,
        subsystem_name: *mut i8,
        name_length: usize,
    ) -> RocmErr;
    pub fn rsmi_dev_drm_render_minor_get(dv_ind: u32, render_minor: *mut u32) -> RocmErr;
    pub fn rsmi_dev_subsystem_vendor_id_get(dv_ind: u32, subsystem_vendor_id: *mut u16) -> RocmErr;
    pub fn rsmi_dev_unique_id_get(dv_ind: u32, unique_id: *mut u64) -> RocmErr;
}
