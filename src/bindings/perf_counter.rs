use libc::c_void;

use crate::error::RocmErr;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    pub fn rsmi_dev_counter_group_supported(dv_ind: u32, group: RsmiEventGroupT) -> RocmErr;
    pub fn rsmi_dev_counter_create(
        dv_ind: u32,
        c_type: RsmiEventTypeT,
        handle: *mut RsmiEventHandleT,
    ) -> RocmErr;
    pub fn rsmi_dev_counter_destroy(handle: RsmiEventHandleT) -> RocmErr;
    // args not used, set null
    pub fn rsmi_counter_control(handle: RsmiEventHandleT, cmd: RsmiCounterCommandT, args: *const c_void) -> RocmErr;
    pub fn rsmi_counter_read(handle: RsmiEventHandleT, value: RsmiCounterValueT) -> RocmErr;
}

pub unsafe fn rsmi_counter_control_uf(handle: RsmiEventHandleT, cmd: RsmiCounterCommandT) ->RocmErr {
    rsmi_counter_control(handle, cmd, std::ptr::null() as *const c_void)
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiEventGroupT {
    RsmiEvntGrpXgmi = 0,
    RsmiEvntGrpXgmiDataOut = 10,
    RsmiEvntGrpInvalid = 0xFFFFFFFF,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiEventTypeT {
    RsmiEvntXgmi0NopTx = RsmiEventGroupT::RsmiEvntGrpXgmi as isize,
    RsmiEvntXgmi0RequestTx,
    RsmiEvntXgmi0ResponseTx,

    // ie, Throughput = BEATS/time_running 10^9  bytes/sec
    RsmiEvntXgmi0BeatsTx,
    RsmiEvntXgmi1NopTx,
    RsmiEvntXgmi1RequestTx,
    RsmiEvntXgmi1ResponseTx,
    RsmiEvntXgmi1BeatsTx,

    /*
     * @brief Events in the RSMI_EVNT_GRP_XGMI_DATA_OUT group measure
     * the number of beats sent on an XGMI link. Each beat represents
     * 32 bytes. RSMI_EVNT_XGMI_DATA_OUT_n represents the number of
     * outbound beats (each representing 32 bytes) on link n.<br><br>
     *
     * XGMI throughput can be calculated by multiplying a event
     * such as ::RSMI_EVNT_XGMI_DATA_OUT_n by 32 and dividing by
     * the time for which event collection occurred,
     * ::rsmi_counter_value_t.time_running (which is in nanoseconds). To get
     * bytes per second, multiply this value by 10<sup>9</sup>.<br>
     * <br>
     * Throughput = BEATS/time_running * 10<sup>9</sup>  (bytes/second)<br>
     */
    // ie, Throughput = BEATS/time_running 10^9  bytes/sec
    RsmiEvntXgmiDataOut0 = RsmiEventGroupT::RsmiEvntGrpXgmiDataOut as isize,
    RsmiEvntXgmiDataOut1,
    RsmiEvntXgmiDataOut2,
    RsmiEvntXgmiDataOut3,
    RsmiEvntXgmiDataOut4,
    RsmiEvntXgmiDataOut5,
}

pub type RsmiEventHandleT = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiCounterCommandT {
    RsmiCntrCmdStart = 0,
    RsmiCntrCmdStop,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct RsmiCounterValueT {
    pub value: u64,            
    pub time_enabled: u64,     
    pub time_running: u64,     
} 