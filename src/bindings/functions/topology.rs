use crate::error::RocmErr;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    pub fn rsmi_topo_get_numa_node_number(dv_ind: u32, numa_node: *mut u32) -> RocmErr;
    pub fn rsmi_topo_get_link_weight(
        dv_ind_src: u32,
        dv_ind_dest: u32,
        weight: *mut u64,
    ) -> RocmErr;
    pub fn rsmi_minmax_bandwidth_get(
        dv_ind_src: u32,
        dv_ind_dest: u32,
        min_bandwidth: *mut u64,
        max_bandwidth: *mut u64,
    ) -> RocmErr;
    pub fn rsmi_topo_get_link_type(
        dv_ind_src: u32,
        dv_ind_dest: u32,
        hops: *mut u64,
        link_type: *mut RsmiIoLinkType,
    ) -> RocmErr;
    pub fn rsmi_is_P2P_accessible(dv_ind_src: u32, dv_ind_dest: u32, acceesible: *mut bool) -> RocmErr;

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
