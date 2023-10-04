use super::RsmiFrequenciesT;
use crate::error::RocmErr;

pub const RSMI_NUM_VOLTAGE_CURVE_POINTS: usize = 3;

#[link(name = "rsmi64", kind = "static")]
extern "C" {

    //performance
    pub fn rsmi_dev_busy_percent_get(dv_ind: u32, percent: *mut u32) -> RocmErr;
    pub fn rsmi_dev_perf_level_get(dv_ind: u32, level: *mut PerformanceLevel) -> RocmErr;
    pub fn rsmi_utilization_count_get(
        dv_ind: u32,
        counter: *mut RsmiUtilizationCounterT,
        count: u32,
        timestamp: *mut u64,
    ) -> RocmErr;
    pub fn rsmi_dev_gpu_clk_freq_get(
        dv_ind: u32,
        clk_type: RsmiClkType,
        clk: *mut RsmiFrequenciesT,
    ) -> RocmErr;
    pub fn rsmi_dev_overdrive_level_get(dv_ind: u32, level: *mut u32) -> RocmErr;
    pub fn rsmi_dev_mem_overdrive_level_get(dv_ind: u32, level: *mut u32) -> RocmErr;
    pub fn rsmi_dev_od_volt_info_get(dv_ind: u32, ov_volt: *mut RsmiOdVoltFreqDataT) -> RocmErr;
    pub fn rsmi_dev_gpu_metrics_info_get(dv_ind: u32, metrics: *mut GpuMetrics) -> RocmErr;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiUtilizationCounterType {
    RsmiCoarseGrainGfxActivity,
    RsmiCoarseGrainMemActivity,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PerformanceLevel {
    Auto,
    Low,
    High,
    Manual,
    StableStd,
    StablePeak,
    StableMinMclk,
    StableMinSclk,
    Determinism,
    Unknown = 0x100,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiClkType {
    RsmiClkTypeSys,
    RsmiClkTypeDf,
    RsmiClkTypeDcef,
    RsmiClkTypeSoc,
    RsmiClkTypeMem,
    RsmiClkTypePcie,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RsmiUtilizationCounterT {
    pub counter_type: RsmiUtilizationCounterType,
    pub value: u64,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct RsmiOdVoltFreqDataT {
    pub curr_sclk_range: RsmiRange,
    pub curr_mclk_range: RsmiRange,
    pub sclk_freq_limits: RsmiRange,
    pub mclk_freq_limits: RsmiRange,
    pub curve: RsmiOdVoltCurveT,
    pub num_regions: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct RsmiRange {
    pub lower_bound: u32,
    pub upper_bound: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct RsmiOdVddcPoint {
    pub frequency: u64,
    pub voltage: u64,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct RsmiOdVoltCurveT {
    pub vc_points: [RsmiOdVddcPoint; RSMI_NUM_VOLTAGE_CURVE_POINTS],
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct GpuMetrics {
    /// metric header
    pub headers: MeticHeader,
    /// Temperature
    pub temperature_edge: u16,
    pub temperature_hotspot: u16,
    pub temperature_mem: u16,
    pub temperature_vrgfx: u16,
    pub temperature_vrsoc: u16,
    pub temperature_vrmem: u16,
    /// Utilization
    pub average_gfx_activity: u16,
    pub average_umc_activity: u16, // memory controller
    pub average_mm_activity: u16,  // UVD or VCN
    /// Power/Energy
    pub average_socket_power: u16,
    pub energy_accumulator: u64,
    /// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
    /// Average clocks
    pub average_gfxclk_frequency: u16,
    /// needs filter
    pub average_socclk_frequency: u16,
    pub average_uclk_frequency: u16,
    pub average_vclk0_frequency: u16,
    pub average_dclk0_frequency: u16,
    pub average_vclk1_frequency: u16,
    pub average_dclk1_frequency: u16,
    /// Current clocks
    pub current_gfxclk: u16,
    /// needs filter
    pub current_socclk: u16,
    /// needs filter
    pub current_uclk: u16,
    pub current_vclk0: u16,
    pub current_dclk0: u16,
    pub current_vclk1: u16,
    pub current_dclk1: u16,

    pub throttle_status: u32,

    pub current_fan_speed: u16,

    pub pcie_link_width: u16,
    pub pcie_link_speed: u16,

    /// not sure what it is for
    /// needs filter
    pub padding: u16,

    pub gfx_activity_acc: u32,
    pub mem_actvity_acc: u32,
    /// needs filter
    pub temperature_hbm: [u16; 4],
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct MeticHeader {
    pub structure_size: u16,
    pub format_revision: u8,
    pub content_revision: u8,
}
