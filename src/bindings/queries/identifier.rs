use libloading::Symbol;

use crate::{error::RocmErr, RawRsmi};

impl RawRsmi {
    pub unsafe fn rsmi_num_monitor_devices(&mut self, num_devices: *mut u32) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(*mut u32) -> RocmErr> =
            match self.lib.get(b"rsmi_num_monitor_devices") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(num_devices)
    }

    pub unsafe fn rsmi_dev_id_get(&mut self, dv_ind: u32, id: *mut u16) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u16) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_id_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, id)
    }

    pub unsafe fn rsmi_dev_revision_get(&mut self, dv_ind: u32, revision: *mut u16) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u16) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_revision_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, revision)
    }
    pub unsafe fn rsmi_dev_sku_get(&mut self, dv_ind: u32, sku: *mut i8) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut i8) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_sku_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, sku)
    }

    pub unsafe fn rsmi_dev_name_get(
        &mut self,
        dv_ind: u32,
        name: *mut i8,
        name_length: usize,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut i8, usize) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_brand_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, name, name_length)
    }

    pub unsafe fn rsmi_dev_vendor_id_get(&mut self, dv_ind: u32, id: *mut u16) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u16) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_vendor_id_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, id)
    }

    pub unsafe fn rsmi_dev_brand_get(
        &mut self,
        dv_ind: u32,
        brand: *mut i8,
        name_length: usize,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut i8, usize) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_brand_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, brand, name_length)
    }

    pub unsafe fn rsmi_dev_vendor_name_get(
        &mut self,
        dv_ind: u32,
        vendor: *mut i8,
        name_length: usize,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut i8, usize) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_vendor_name_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, vendor, name_length)
    }

    pub unsafe fn rsmi_dev_vram_vendor_get(
        &mut self,
        dv_ind: u32,
        vendor: *mut i8,
        name_length: usize,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut i8, usize) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_vram_vendor_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, vendor, name_length)
    }

    pub unsafe fn rsmi_dev_serial_number_get(
        &mut self,
        dv_ind: u32,
        serial_number: *mut i8,
        name_length: usize,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut i8, usize) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_serial_number_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, serial_number, name_length)
    }

    pub unsafe fn rsmi_dev_subsystem_id_get(&mut self, dv_ind: u32, id: *mut u16) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u16) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_subsystem_id_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, id)
    }

    pub unsafe fn rsmi_dev_subsystem_name_get(
        &mut self,
        dv_ind: u32,
        subsystem_name: *mut i8,
        name_length: usize,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut i8, usize) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_subsystem_name_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, subsystem_name, name_length)
    }

    pub unsafe fn rsmi_dev_drm_render_minor_get(
        &mut self,
        dv_ind: u32,
        render_minor: *mut u32,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u32) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_drm_render_minor_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, render_minor)
    }

    pub unsafe fn rsmi_dev_subsystem_vendor_id_get(
        &mut self,
        dv_ind: u32,
        subsystem_vendor_id: *mut u16,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u16) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_subsystem_vendor_id_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, subsystem_vendor_id)
    }

    pub unsafe fn rsmi_dev_unique_id_get(&mut self, dv_ind: u32, unique_id: *mut u64) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_unique_id_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, unique_id)
    }

    pub unsafe fn rsmi_dev_xgmi_physical_id_get(&mut self, dv_ind: u32, id: *mut u16) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u16) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_xgmi_physical_id_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, id)
    }
}
