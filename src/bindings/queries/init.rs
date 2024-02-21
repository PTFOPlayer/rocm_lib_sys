use crate::error::RocmErr;

const PATH: &str = "/opt/rocm/lib/librocm_smi64.so";

use libloading::{Library, Symbol};
pub unsafe fn rsmi_init(init_status: u32) -> Result<RocmErr, libloading::Error> {
    let lib = Library::new(PATH)?;
    let function: Symbol<unsafe extern "C" fn(u32) -> RocmErr> = lib.get(b"rsmi_init")?;
    Ok(function(init_status))
}

pub unsafe fn rsmi_shut_down() -> Result<RocmErr, libloading::Error> {
    let lib = Library::new(PATH)?;
    let function: Symbol<unsafe extern "C" fn() -> RocmErr> = lib.get(b"rsmi_shut_down")?;
    Ok(function())
}