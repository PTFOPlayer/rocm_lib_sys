use crate::{error::RocmErr, function_creator, RawRsmi};

impl RawRsmi {
    pub unsafe fn rsmi_num_monitor_devices(&mut self, num_devices: *mut u32) -> RocmErr {
        function_creator!(self, b"rsmi_num_monitor_devices", <*mut u32>, (num_devices))
    }

    pub unsafe fn rsmi_dev_id_get(&mut self, dv_ind: u32, id: *mut u16) -> RocmErr {
        function_creator!(self, b"rsmi_dev_id_get", <u32, *mut u16>, (dv_ind, id))
    }

    pub unsafe fn rsmi_dev_name_get(
        &mut self,
        dv_ind: u32,
        name: *mut i8,
        name_length: usize,
    ) -> RocmErr {
        function_creator!(self, b"rsmi_dev_name_get", <u32, *mut i8, usize>, (dv_ind, name, name_length) )
    }

    pub unsafe fn rsmi_dev_vendor_id_get(&mut self, dv_ind: u32, id: *mut u16) -> RocmErr {
        function_creator!(self, b"rsmi_dev_vendor_id_get", <u32, *mut u16>, (dv_ind, id))
    }

    pub unsafe fn rsmi_dev_brand_get(
        &mut self,
        dv_ind: u32,
        brand: *mut i8,
        name_length: usize,
    ) -> RocmErr {
        function_creator!(self, b"rsmi_dev_brand_get", <u32, *mut i8, usize>, (dv_ind, brand, name_length))
    }

    pub unsafe fn rsmi_dev_vendor_name_get(
        &mut self,
        dv_ind: u32,
        vendor: *mut i8,
        name_length: usize,
    ) -> RocmErr {
        function_creator!(self, b"rsmi_dev_vendor_name_get", <u32, *mut i8, usize>, (dv_ind, vendor, name_length))
    }

    pub unsafe fn rsmi_dev_vram_vendor_get(
        &mut self,
        dv_ind: u32,
        vendor: *mut i8,
        name_length: usize,
    ) -> RocmErr {
        function_creator!(self, b"rsmi_dev_vram_vendor_get", <u32, *mut i8, usize>, (dv_ind, vendor, name_length))
    }

    pub unsafe fn rsmi_dev_serial_number_get(
        &mut self,
        dv_ind: u32,
        serial_number: *mut i8,
        name_length: usize,
    ) -> RocmErr {
        function_creator!(self, b"rsmi_dev_serial_number_get", <u32, *mut i8, usize>, (dv_ind, serial_number, name_length))
    }

    pub unsafe fn rsmi_dev_subsystem_id_get(&mut self, dv_ind: u32, id: *mut u16) -> RocmErr {
        function_creator!(self, b"rsmi_dev_subsystem_id_get", <u32, *mut u16>, (dv_ind, id))
    }

    pub unsafe fn rsmi_dev_subsystem_name_get(
        &mut self,
        dv_ind: u32,
        subsystem_name: *mut i8,
        name_length: usize,
    ) -> RocmErr {
        function_creator!(self, b"rsmi_dev_subsystem_name_get", <u32, *mut i8, usize>, (dv_ind, subsystem_name, name_length))
    }

    pub unsafe fn rsmi_dev_drm_render_minor_get(
        &mut self,
        dv_ind: u32,
        render_minor: *mut u32,
    ) -> RocmErr {
        function_creator!(self, b"rsmi_dev_drm_render_minor_get", <u32, *mut u32>, (dv_ind, render_minor))
    }

    pub unsafe fn rsmi_dev_subsystem_vendor_id_get(
        &mut self,
        dv_ind: u32,
        subsystem_vendor_id: *mut u16,
    ) -> RocmErr {
        function_creator!(self, b"rsmi_dev_subsystem_vendor_id_get", <u32, *mut u16>, (dv_ind, subsystem_vendor_id))
    }

    pub unsafe fn rsmi_dev_unique_id_get(&mut self, dv_ind: u32, unique_id: *mut u64) -> RocmErr {
        function_creator!(self, b"rsmi_dev_unique_id_get", <u32, *mut u64>, (dv_ind, unique_id))
    }
}
