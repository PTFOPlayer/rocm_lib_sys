pub mod bindings;
pub mod error;

#[cfg(test)]
mod test {
    use crate::{
        bindings::{rsmi_dev_brand_get, rsmi_init, rsmi_shut_down},
        error::RocmErr,
    };

    #[test]
    fn minor_test() {
        let ret = unsafe { rsmi_init(0) };
        assert_eq!(ret, RocmErr::RsmiStatusSuccess);

        let buff = unsafe { libc::malloc(64).cast() };
        let ret = unsafe { rsmi_dev_brand_get(0, buff, 64) };
        assert_eq!(ret, RocmErr::RsmiStatusSuccess);
        let temp = unsafe { std::ffi::CString::from_raw(buff) };
        println!("{:?}", temp.to_string_lossy().to_string());
    
        let ret = unsafe { rsmi_shut_down() };
        assert_eq!(ret, RocmErr::RsmiStatusSuccess)
    }
}
