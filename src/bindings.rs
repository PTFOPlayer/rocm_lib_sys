use crate::error::RocmErr;
// Maximum possible value for fan speed. Should be used as the denominator
// when determining fan speed percentage.
pub const RSMI_MAX_FAN_SPEED: usize = 255;

mod queries;
mod functions;
pub use identifier::*;
pub use functions::*;

use queries::*;

pub use init::*;

pub use pcie::*;

pub use power::*;

pub use physical::*;

pub use memory::*;

pub use performance::*;

pub use version::*;

pub use error::*;

pub use perf_counter::*;

pub use sys_info::*;

pub use topology::*;

pub use supported_fn::*;
// #[link(name = "rsmi64", kind = "static")]
// extern "C" {}

impl ToString for PerformanceLevel {
    fn to_string(&self) -> String {
        match self {
            PerformanceLevel::Auto => "performance level: Auto".to_owned(),
            PerformanceLevel::Low => "performance level: Low".to_owned(),
            PerformanceLevel::High => "performance level: High".to_owned(),
            PerformanceLevel::Manual => "performance level: Manual".to_owned(),
            PerformanceLevel::StableStd => "performance level: Stable Std".to_owned(),
            PerformanceLevel::StablePeak => "performance level: Stable Peak".to_owned(),
            PerformanceLevel::StableMinMclk => "performance level: Stable Min MClk".to_owned(),
            PerformanceLevel::StableMinSclk => "performance level: Stable Min SClk".to_owned(),
            PerformanceLevel::Determinism => "performance level: Determinism".to_owned(),
            PerformanceLevel::Unknown => "performance level: Unknown".to_owned(),
        }
    }
}

#[inline(always)]
pub unsafe fn string_from_fn(
    dv_ind: u32,
    name_size: usize,
    f: unsafe extern "C" fn(u32, *mut i8, usize) -> RocmErr,
) -> Result<String, RocmErr> {
    let buff = libc::malloc(name_size).cast();
    f(dv_ind, buff, name_size).try_err()?;
    let temp = std::ffi::CString::from_raw(buff);
    return Ok(temp.to_string_lossy().to_string());
}
