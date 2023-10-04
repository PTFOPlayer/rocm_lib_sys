use crate::error::RocmErr;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    pub fn rsmi_dev_ecc_count_get(
        dv_ind: u32,
        block: RsmiGpuBlockT,
        ec: *mut RsmiErrorCountT,
    ) -> RocmErr;
    pub fn rsmi_dev_ecc_enabled_get(dv_ind: u32, enablet_bits: u64) -> RocmErr;
    pub fn rsmi_dev_ecc_status_get(
        dv_ind: u32,
        block: RsmiGpuBlockT,
        state: *mut RsmiRasErrStateT,
    ) -> RocmErr;
    pub fn rsmi_status_string(status: RocmErr, status_string: *mut *const i8);
}

#[allow(conflicting_repr_hints)]
#[repr(usize)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiGpuBlockT {
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
pub enum RsmiRasErrStateT {
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
pub struct RsmiErrorCountT {
    pub correctable_err: u64,
    pub uncorrectable_err: u64,
}
