use error::RocmErr;
use libloading::{Library, Symbol};

pub mod bindings;
pub mod error;

const PATH: &str = "/opt/rocm/lib/librocm_smi64.so";

#[derive(Debug)]
pub struct RawRsmi {
    path: &'static str,
    lib: Library,
}

impl RawRsmi {
    pub unsafe fn new(init_status: u32) -> Result<RawRsmi, RocmErr> {
        let lib = Library::new(PATH)?;
        let f: Symbol<unsafe extern "C" fn(u32) -> RocmErr> = lib.get(b"rsmi_init")?;
        f(init_status).try_err()?;
        Ok(Self { path: PATH, lib })
    }

    pub unsafe fn with_path(init_status: u32, path: &'static str) -> Result<RawRsmi, RocmErr> {
        let lib = Library::new(path)?;
        let f: Symbol<unsafe extern "C" fn(u32) -> RocmErr> = lib.get(b"rsmi_init")?;
        f(init_status).try_err()?;
        Ok(Self { path, lib })
    }

    pub fn get_path(&self) -> &'static str {
        self.path
    }
}

impl Drop for RawRsmi {
    fn drop(&mut self) {
        unsafe {
            let res: Result<Symbol<unsafe extern "C" fn() -> RocmErr>, RocmErr> = self
                .lib
                .get(b"rsmi_shut_down")
                .map_err(|_| RocmErr::RsmiLibLoadingError);
            match res {
                Ok(f) => {
                    let _res = f().try_err();
                }
                Err(_) => {}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        bindings::{RsmiFwBlock, RsmiVersion},
        error::RocmErr,
        RawRsmi,
    };
    use std::mem::size_of;

    #[test]
    fn minor_test() -> Result<(), RocmErr> {
        unsafe {
            let mut rrsmi = RawRsmi::new(0)?;
            let buff = libc::malloc(size_of::<i8>() * 64).cast();
            rrsmi.rsmi_dev_brand_get(0, buff, 64).try_err()?;

            let temp = std::ffi::CString::from_raw(buff);
            println!("{:?}", temp.to_string_lossy().to_string());
        }
        Ok(())
    }

    #[test]
    fn processes() -> Result<(), RocmErr> {
        unsafe {
            let mut rrsmi = RawRsmi::new(0)?;

            let procs = vec![].as_mut_ptr();
            let mut num_items = 0u32;
            rrsmi
                .rsmi_compute_process_info_get(procs, &mut num_items as *mut u32)
                .try_err()?;

            let slice = std::slice::from_raw_parts_mut(procs, num_items as usize);
            println!("num procs:{}", num_items);
            for e in slice {
                println!("{:?}", e);
            }
        }
        Ok(())
    }

    #[test]
    fn firmware() -> Result<(), RocmErr> {
        unsafe {
            let mut rrsmi = RawRsmi::new(0)?;

            let mut rsmi_v: RsmiVersion = RsmiVersion {
                major: 0,
                minor: 0,
                patch: 0,
                build: &mut 0i8,
            };

            rrsmi
                .rsmi_version_get(&mut rsmi_v as *mut RsmiVersion)
                .try_err()?;
            println!("Rsmi version: {:?}", rsmi_v);

            let mut v = 0u64;
            for item in RsmiFwBlock::enum_iterator() {
                match rrsmi
                    .rsmi_dev_firmware_version_get(0, item, &mut v as *mut u64)
                    .try_err()
                {
                    Ok(_) => println!("firmware version {:?}:{}", item, v),
                    Err(_) => {}
                }
            }
        }

        Ok(())
    }

    // #[test]
    // fn bios() -> Result<(), RocmErr> {
    //     unsafe {
    //         rsmi_init(0)?.try_err()?;

    //         let data = string_from_fn(0, 128, rsmi_dev_vbios_version_get);
    //         println!("bios:{:?}", data);

    //         rsmi_shut_down()?.try_err()?;
    //     }
    //     Ok(())
    // }

    // #[test]
    // fn supported_fn() -> Result<(), RocmErr> {
    //     unsafe {
    //         rsmi_init(0)?.try_err()?;
    //         let mut handle = RsmiFuncIdIterHandleT::new();
    //         let hdl_ptr = &mut handle as *mut RsmiFuncIdIterHandleT;
    //         rsmi_dev_supported_func_iterator_open(0, hdl_ptr).try_err()?;

    //         let mut value = RsmiFuncIdValueT::default();
    //         let val_ptr = &mut value as *mut RsmiFuncIdValueT;

    //         let mut var_handle = RsmiFuncIdIterHandleT::new();
    //         let var_ptr = &mut var_handle as *mut RsmiFuncIdIterHandleT;

    //         let mut sub_var_handle = RsmiFuncIdIterHandleT::new();
    //         let svar_ptr = &mut sub_var_handle as *mut RsmiFuncIdIterHandleT;

    //         loop {
    //             let mut res: RocmErr;
    //             rsmi_func_iter_value_get(handle, val_ptr);
    //             let buff = libc::malloc(128 * size_of::<i8>()).cast::<i8>();
    //             value.name.cast::<i8>().copy_to_nonoverlapping(buff, 128);
    //             let temp = std::ffi::CString::from_raw(buff);
    //             let fn_name = temp.to_string_lossy().to_string();
    //             println!("{}", fn_name);

    //             res = rsmi_dev_supported_variant_iterator_open(handle, var_ptr);
    //             if res != RocmErr::RsmiStatusNoData {
    //                 print!("\tVariants/Monitors: ");
    //                 loop {
    //                     rsmi_func_iter_value_get(var_handle, val_ptr);
    //                     if value.id == RSMI_DEFAULT_VARIANT {
    //                         print!("Default Variant ");
    //                     } else {
    //                         print!("{} ", value.id);
    //                     }

    //                     print!(" (");

    //                     res = rsmi_dev_supported_variant_iterator_open(var_handle, svar_ptr);
    //                     if res != RocmErr::RsmiStatusNoData {
    //                         loop {
    //                             rsmi_func_iter_value_get(sub_var_handle, val_ptr);
    //                             print!("{}, ", value.id);

    //                             res = rsmi_func_iter_next(sub_var_handle);

    //                             if res == RocmErr::RsmiStatusNoData {
    //                                 break;
    //                             }
    //                         }
    //                         rsmi_dev_supported_func_iterator_close(svar_ptr);
    //                     }

    //                     print!(") ");

    //                     res = rsmi_func_iter_next(var_handle);

    //                     if res == RocmErr::RsmiStatusNoData {
    //                         break;
    //                     }
    //                 }
    //                 println!();

    //                 rsmi_dev_supported_func_iterator_close(var_ptr);
    //             }

    //             res = rsmi_func_iter_next(handle);
    //             if res == RocmErr::RsmiStatusNoData {
    //                 break;
    //             }
    //         }
    //     }

    //     Ok(())
    // }
}
