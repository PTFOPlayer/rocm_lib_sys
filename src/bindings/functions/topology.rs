use libloading::Symbol;

use crate::{error::RocmErr, RawRsmi};

impl RawRsmi {
    pub unsafe fn rsmi_topo_get_numa_node_number(
        &mut self,
        dv_ind: u32,
        numa_node: *mut u32,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u32) -> RocmErr> =
            match self.lib.get(b"rsmi_topo_get_numa_node_number") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, numa_node)
    }

    pub unsafe fn rsmi_topo_get_link_weight(
        &mut self,
        dv_ind_src: u32,
        dv_ind_dest: u32,
        weight: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, u32, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_topo_get_link_weight") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind_src, dv_ind_dest, weight)
    }

    pub unsafe fn rsmi_minmax_bandwidth_get(
        &mut self,
        dv_ind_src: u32,
        dv_ind_dest: u32,
        min_bandwidth: *mut u64,
        max_bandwidth: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, u32, *mut u64, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_minmax_bandwidth_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind_src, dv_ind_dest, min_bandwidth, max_bandwidth)
    }

    pub unsafe fn rsmi_topo_get_link_type(
        &mut self,
        dv_ind_src: u32,
        dv_ind_dest: u32,
        hops: *mut u64,
        link_type: *mut RsmiIoLinkType,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, u32, *mut u64, *mut RsmiIoLinkType) -> RocmErr> =
            match self.lib.get(b"rsmi_topo_get_link_type") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind_src, dv_ind_dest, hops, link_type)
    }

    #[allow(non_snake_case)]
    pub unsafe fn rsmi_is_P2P_accessible(
        &mut self,
        dv_ind_src: u32,
        dv_ind_dest: u32,
        acceesible: *mut bool,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, u32, *mut bool) -> RocmErr> =
            match self.lib.get(b"rsmi_is_P2P_accessible") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind_src, dv_ind_dest, acceesible)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiIoLinkType {
    RsmiIolinkTypeUndefined = 0,
    RsmiIolinkTypePciexpress = 1,
    RsmiIolinkTypeXgmi = 2,
    RsmiIolinkTypeNumiolinktypes,
    RsmiIolinkTypeSize = 0xFFFFFFFF,
}
