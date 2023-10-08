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

    #[test]
    fn processes() -> Result<(), RocmErr> {
        unsafe {
            let procs = vec![].as_mut_ptr();
            let mut num_items = 0u32;
            rsmi_compute_process_info_get(procs, &mut num_items as *mut u32).try_err()?;

            let slice = std::slice::from_raw_parts_mut(procs, num_items as usize);
            println!("num procs:{}", num_items);
            for e in slice {
                println!("{:?}", e);
            }
        }        
        Ok(())
    }

    #[test]
    fn firmware() ->  Result<(), RocmErr> {
        unsafe {

            rsmi_init(0).try_err()?;
            
            let mut v = 0u64;
            for item in RsmiFwBlockT::enum_iterator() {
                rsmi_dev_firmware_version_get(0, item, &mut v as * mut u64 ).try_err()?;
                println!("firmware version {:?}:{}", item, v);
            }
            
            rsmi_shut_down().try_err()?;
        }

        Ok(())
    }

    #[test]
    fn bios() -> Result<(), RocmErr> {
        unsafe {
            rsmi_init(0).try_err()?;

            let data = string_from_fn(0, 128, rsmi_dev_vbios_version_get);
            println!("bios:{:?}", data);

            rsmi_shut_down().try_err()?;
        }
        Ok(())
    }
}
