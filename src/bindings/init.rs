use crate::error::RocmErr;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    pub fn rsmi_init(init_status: u32) -> RocmErr;
    pub fn rsmi_shut_down() -> RocmErr;
}
