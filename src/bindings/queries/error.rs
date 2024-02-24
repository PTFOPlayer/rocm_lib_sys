use libloading::Symbol;
use crate::{error::RocmErr, RawRsmi};

impl RawRsmi {
    pub unsafe fn rsmi_dev_ecc_count_get(
        &mut self,
        dv_ind: u32,
        block: RsmiGpuBlock,
        ec: *mut RsmiErrorCount,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, RsmiGpuBlock, *mut RsmiErrorCount) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_ecc_count_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, block, ec)
    }

    pub unsafe fn rsmi_dev_ecc_enabled_get(
        &mut self,
        dv_ind: u32,
        enablet_bits: *mut u64,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, *mut u64) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_ecc_enabled_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, enablet_bits)
    }

    pub unsafe fn rsmi_dev_ecc_status_get(
        &mut self,
        dv_ind: u32,
        block: RsmiGpuBlock,
        state: *mut RsmiRasErrState,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(u32, RsmiGpuBlock, *mut RsmiRasErrState) -> RocmErr> =
            match self.lib.get(b"rsmi_dev_ecc_status_get") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(dv_ind, block, state)
    }

    pub unsafe fn rsmi_status_string(
        &mut self,
        status: RocmErr,
        status_string: *mut *const i8,
    ) -> RocmErr {
        let f: Symbol<unsafe extern "C" fn(RocmErr, *mut *const i8) -> RocmErr> =
            match self.lib.get(b"rsmi_status_string") {
                Ok(res) => res,
                Err(err) => return err.into(),
            };
        f(status, status_string)
    }
}

#[allow(conflicting_repr_hints)]
#[repr(usize)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiGpuBlock {
    RsmiGpuBlockInvalid = 0x0000000000000000,
    RsmiGpuBlockUmc = 0x0000000000000001,
    RsmiGpuBlockSdma = 0x0000000000000002,
    RsmiGpuBlockGfx = 0x0000000000000004,
    RsmiGpuBlockMmhub = 0x0000000000000008,
    RsmiGpuBlockAthub = 0x0000000000000010,
    RsmiGpuBlockPcieBif = 0x0000000000000020,
    RsmiGpuBlockHdp = 0x0000000000000040,
    RsmiGpuBlockXgmiWafl = 0x0000000000000080,
    RsmiGpuBlockDf = 0x0000000000000100,
    RsmiGpuBlockSmn = 0x0000000000000200,
    RsmiGpuBlockSem = 0x0000000000000400,
    RsmiGpuBlockMp0 = 0x0000000000000800,
    RsmiGpuBlockMp1 = 0x0000000000001000,
    RsmiGpuBlockFuse = 0x0000000000002000,
    RsmiGpuBlockReserved = 0x8000000000000000,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiRasErrState {
    RsmiRasErrStateNone = 0,
    RsmiRasErrStateDisabled,
    RsmiRasErrStateParity,
    RsmiRasErrStateSingC,
    RsmiRasErrStateMultUc,
    RsmiRasErrStatePoison,
    RsmiRasErrStateEnabled,
    RsmiRasErrStateInvalid = 0xFFFFFFFF,
}

#[repr(C)]
#[derive(Debug)]
pub struct RsmiErrorCount {
    pub correctable_err: u64,
    pub uncorrectable_err: u64,
}
