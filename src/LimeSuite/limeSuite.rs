
use crate::LimeSuite::lib::*;
use std::ffi::CStr;

pub fn get_device_list() -> Result<std::vec::Vec<String>, ()> {
    let mut list = std::vec::Vec::<lms_info_str_t>::with_capacity(5);
    
        let num_devices: i32 = unsafe { LMS_GetDeviceList(list.as_mut_ptr()) };

    if num_devices < 0 {
        return Err(());
    }
    let num_devices: usize = num_devices as usize;
    let mut res = std::vec::Vec::<String>::new();
    for i in 0..num_devices {
        unsafe{
                                            res.push( CStr::from_ptr(list[i].as_mut_ptr()).
                                            to_str().
                                            unwrap().
                                            to_owned());
        }
    }
    Ok(res)
}