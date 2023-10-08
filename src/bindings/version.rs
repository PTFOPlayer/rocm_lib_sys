use crate::error::RocmErr;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    pub fn rsmi_version_get(version: *mut RsmiVersion) -> RocmErr;
    pub fn rsmi_version_str_get(
        component: RsmiSwComponentT,
        ver_str: *mut i8,
        length: u32,
    ) -> RocmErr;
    pub fn rsmi_dev_vbios_version_get(dv_ind: u32, vbios: *mut i8, length: usize) -> RocmErr;
    pub fn rsmi_dev_firmware_version_get(
        dv_ind: u32,
        block: RsmiFwBlockT,
        version: *mut u64,
    ) -> RocmErr;
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RsmiVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub build: *mut i8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiSwComponentT {
    RsmiSwCompDriver,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiFwBlockT {
    RsmiFwBlockAsd,
    RsmiFwBlockCe,
    RsmiFwBlockDmcu,
    RsmiFwBlockMc,
    RsmiFwBlockMe,
    RsmiFwBlockMec,
    RsmiFwBlockMec2,
    RsmiFwBlockPfp,
    RsmiFwBlockRlc,
    RsmiFwBlockRlcSrlc,
    RsmiFwBlockRlcSrlg,
    RsmiFwBlockRlcSrls,
    RsmiFwBlockSdma,
    RsmiFwBlockSdma2,
    RsmiFwBlockSmc,
    RsmiFwBlockSos,
    RsmiFwBlockTaRas,
    RsmiFwBlockTaXgmi,
    RsmiFwBlockUvd,
    RsmiFwBlockVce,
    RsmiFwBlockVcn,
}

impl RsmiFwBlockT {
    pub fn enum_iterator() -> std::array::IntoIter<RsmiFwBlockT, 21>  {
        [    RsmiFwBlockT::RsmiFwBlockAsd,
        RsmiFwBlockT::RsmiFwBlockCe,
        RsmiFwBlockT::RsmiFwBlockDmcu,
        RsmiFwBlockT::RsmiFwBlockMc,
        RsmiFwBlockT::RsmiFwBlockMe,
        RsmiFwBlockT::RsmiFwBlockMec,
        RsmiFwBlockT::RsmiFwBlockMec2,
        RsmiFwBlockT::RsmiFwBlockPfp,
        RsmiFwBlockT::RsmiFwBlockRlc,
        RsmiFwBlockT::RsmiFwBlockRlcSrlc,
        RsmiFwBlockT::RsmiFwBlockRlcSrlg,
        RsmiFwBlockT::RsmiFwBlockRlcSrls,
        RsmiFwBlockT::RsmiFwBlockSdma,
        RsmiFwBlockT::RsmiFwBlockSdma2,
        RsmiFwBlockT::RsmiFwBlockSmc,
        RsmiFwBlockT::RsmiFwBlockSos,
        RsmiFwBlockT::RsmiFwBlockTaRas,
        RsmiFwBlockT::RsmiFwBlockTaXgmi,
        RsmiFwBlockT::RsmiFwBlockUvd,
        RsmiFwBlockT::RsmiFwBlockVce,
        RsmiFwBlockT::RsmiFwBlockVcn].into_iter()
    }
}
