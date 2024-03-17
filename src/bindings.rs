// Maximum possible value for fan speed. Should be used as the denominator
// when determining fan speed percentage.
pub const RSMI_MAX_FAN_SPEED: usize = 255;

mod queries;
mod functions;
mod controls;

pub use functions::*;

use queries::*;

pub use pcie::*;

pub use physical::*;

pub use memory::*;

pub use performance::*;

pub use version::*;

pub use error::*;

pub use perf_counter::*;

pub use sys_info::*;

pub use topology::*;

pub use supported_fn::*;

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