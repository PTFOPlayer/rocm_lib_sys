pub mod bindings;
pub mod error;


#[cfg(test)]
mod test {
    use crate::bindings::*;
    use std::mem::size_of;

    #[test]
    fn name_brand_rev_sku_test() {
        unsafe {
            rsmi_init(0);
            let brand_buff = libc::malloc(size_of::<i8>() * 64).cast();
            assert_eq!(rsmi_dev_brand_get(0, brand_buff, 64), 0);
            let temp = std::ffi::CString::from_raw(brand_buff);
            println!("brand: {:?}", temp.to_string_lossy().to_string());

            let name_buff = libc::malloc(size_of::<i8>() * 128).cast();
            assert_eq!(rsmi_dev_name_get(0, name_buff, 128), 0);
            let temp = std::ffi::CString::from_raw(name_buff);
            println!("name: {:?}", temp.to_string_lossy().to_string());

            let mut rev = 0u16;

            assert_eq!(rsmi_dev_revision_get(0, &mut rev as *mut u16), 0);
            println!("revision: 0x{:x?}", rev);
            // not supported on rx 7600
            // let sku = libc::malloc(size_of::<i8>() * 128).cast();
            // assert_eq!(rsmi_dev_sku_get(0, sku), 0);
            // let temp = std::ffi::CString::from_raw(sku);
            // println!("sku: {:?}", temp.to_string_lossy().to_string());
        }

    }

    // #[test]
    // fn processes() -> Result<(), RocmErr> {
    //     unsafe {
    //         let mut rrsmi = RawRsmi::new(0)?;

    //         let procs = vec![].as_mut_ptr();
    //         let mut num_items = 0u32;
    //         rrsmi
    //             .rsmi_compute_process_info_get(procs, &mut num_items as *mut u32)
    //             .try_err()?;

    //         let slice = std::slice::from_raw_parts_mut(procs, num_items as usize);
    //         println!("num procs:{}", num_items);
    //         for e in slice {
    //             println!("{:?}", e);
    //         }
    //     }
    //     Ok(())
    // }

    // #[test]
    // fn firmware() -> Result<(), RocmErr> {
    //     unsafe {
    //         let mut rrsmi = RawRsmi::new(0)?;

    //         let mut rsmi_v: RsmiVersion = RsmiVersion {
    //             major: 0,
    //             minor: 0,
    //             patch: 0,
    //             build: &mut 0i8,
    //         };

    //         rrsmi
    //             .rsmi_version_get(&mut rsmi_v as *mut RsmiVersion)
    //             .try_err()?;
    //         println!("Rsmi version: {:?}", rsmi_v);

    //         let mut v = 0u64;
    //         for item in RsmiFwBlock::enum_iterator() {
    //             match rrsmi
    //                 .rsmi_dev_firmware_version_get(0, item, &mut v as *mut u64)
    //                 .try_err()
    //             {
    //                 Ok(_) => println!("firmware version {:?}:{}", item, v),
    //                 Err(_) => {}
    //             }
    //         }
    //     }

    //     Ok(())
    // }

    // #[test]
    // fn activity_test() -> Result<(), RocmErr> {
    //     unsafe {
    //         let mut rrsmi = RawRsmi::new(0)?;

    //         let mut acc = 0u16;
    //         rrsmi
    //             .rsmi_dev_activity_avg_mm_get(0, &mut acc as *mut u16)
    //             .try_err()?;

    //         println!("avg acc:{}", acc);
    //     }
    //     Ok(())
    // }

    // #[test]
    // fn gpu_metrics_test() -> Result<(), RocmErr> {
    //     unsafe {
    //         let mut rrsmi = RawRsmi::new(0)?;

    //         let mut metrics = RsmiGpuMetrics::default();
    //         rrsmi
    //             .rsmi_dev_gpu_metrics_info_get(0, &mut metrics as *mut RsmiGpuMetrics)
    //             .try_err()?;

    //         println!("metrics:{:?}", metrics);
    //     }
    //     Ok(())
    // }

    // #[test]
    // fn power_profile_test() -> Result<(), RocmErr> {
    //     unsafe {
    //         let mut rrsmi = RawRsmi::new(0)?;

    //         let mut profile = RsmiPowerProfileStatus::default();

    //         println!("setting power profile: {:?}",rrsmi
    //         .rsmi_dev_power_profile_set_v0(0, crate::bindings::RsmiPowerProfilePresetMasks::RsmiPwrProfPrstBootupDefault)
    //         .try_err());
    //         rrsmi
    //             .rsmi_dev_power_profile_presets_get(0, 0, &mut profile as *mut RsmiPowerProfileStatus)
    //             .try_err()?;

    //         println!("power profile: {:?}", profile.current);
    //         println!("bitmap: {:b}", profile.available_profiles)

    //     }
    //     Ok(())
    // }
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
