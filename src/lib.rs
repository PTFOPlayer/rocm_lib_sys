pub mod bindings;
pub mod error;

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use crate::{bindings::*, error::RocmErr};

    #[test]
    fn minor_test() -> Result<(), RocmErr> {
        unsafe {
            rsmi_init(0).try_err()?;

            let buff = libc::malloc(size_of::<i8>() * 64).cast();
            rsmi_dev_brand_get(0, buff, 64).try_err()?;

            let temp = std::ffi::CString::from_raw(buff);
            println!("{:?}", temp.to_string_lossy().to_string());

            rsmi_shut_down().try_err()?;
        }

        Ok(())
    }
}
