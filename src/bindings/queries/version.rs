use libloading::Symbol;

use crate::{error::RocmErr, RawRsmi};

impl RawRsmi {
    pub unsafe fn rsmi_version_get(&mut self, version: *mut RsmiVersion) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(*mut RsmiVersion) -> RocmErr> =
            match self.lib.get(b"rsmi_version_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(version)
    }

    pub unsafe fn rsmi_version_str_get(
        &mut self,
        component: RsmiSwComponent,
        ver_str: *mut i8,
        length: u32,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(RsmiSwComponent, *mut i8, u32) -> RocmErr> =
            match self.lib.get(b"rsmi_version_str_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(component, ver_str, length)
    }
    pub unsafe fn rsmi_dev_vbios_version_get(
        &mut self,
        dv_ind: u32,
        vbios: *mut i8,
        length: u32,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut i8, u32) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_vbios_version_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, vbios, length)
    }
    pub unsafe fn rsmi_dev_firmware_version_get(
        &mut self,
        dv_ind: u32,
        block: RsmiFwBlock,
        version: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, RsmiFwBlock, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_firmware_version_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, block, version)
    }
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
pub enum RsmiSwComponent {
    RsmiSwCompDriver,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiFwBlock {
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

impl RsmiFwBlock {
    pub fn enum_iterator() -> std::array::IntoIter<RsmiFwBlock, 21> {
        [
            RsmiFwBlock::RsmiFwBlockAsd,
            RsmiFwBlock::RsmiFwBlockCe,
            RsmiFwBlock::RsmiFwBlockDmcu,
            RsmiFwBlock::RsmiFwBlockMc,
            RsmiFwBlock::RsmiFwBlockMe,
            RsmiFwBlock::RsmiFwBlockMec,
            RsmiFwBlock::RsmiFwBlockMec2,
            RsmiFwBlock::RsmiFwBlockPfp,
            RsmiFwBlock::RsmiFwBlockRlc,
            RsmiFwBlock::RsmiFwBlockRlcSrlc,
            RsmiFwBlock::RsmiFwBlockRlcSrlg,
            RsmiFwBlock::RsmiFwBlockRlcSrls,
            RsmiFwBlock::RsmiFwBlockSdma,
            RsmiFwBlock::RsmiFwBlockSdma2,
            RsmiFwBlock::RsmiFwBlockSmc,
            RsmiFwBlock::RsmiFwBlockSos,
            RsmiFwBlock::RsmiFwBlockTaRas,
            RsmiFwBlock::RsmiFwBlockTaXgmi,
            RsmiFwBlock::RsmiFwBlockUvd,
            RsmiFwBlock::RsmiFwBlockVce,
            RsmiFwBlock::RsmiFwBlockVcn,
        ]
        .into_iter()
    }
}
