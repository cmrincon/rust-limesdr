#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::LimeSuite::lib::*;
use std::ffi::CStr;

pub fn get_device_list() -> Result<std::vec::Vec<String>, ()> {
    let mut list: [lms_info_str_t; 5] = [[0; 256]; 5];
    
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

struct Device {
    dev: *mut lms_device_t,
}

impl Device {
    fn open(dev_info: &str ) -> Result<Device,()> {
        let mut dev: *mut lms_device_t = std::ptr::null_mut();
        let string: std::ffi::CString = 
            std::ffi::CString::new(dev_info).expect("String failed");
        match unsafe {
            LMS_Open(std::ptr::addr_of_mut!(dev), string.as_ptr() as *const [u8;256], std::ptr::null_mut())
        } {
            LMS_SUCCESS => Ok(Device {
                dev: dev 
            }),
            _ => return Err(()),
        } 
    }

    fn init(&self) -> Result<(), ()> {
        match unsafe {
            LMS_Init(self.dev)} {
                LMS_SUCCESS => Ok(()),
                _ => return Err(()),
            }
    }

    fn getNumChannels(&self, dir_tx: bool) -> Result<i32,()> {
        let res: i32 = unsafe {
            LMS_GetNumChannels(self.dev, &dir_tx)
        };

        if res >= LMS_SUCCESS {
            Ok(res)
        }
        else {
            return Err(());
        }
    }

    fn LMS_EnableChannel(&self, dir_tx: bool, 
        chan: usize, enabled: bool) -> Result<(),()> {

        let res = unsafe {
            LMS_EnableChannel(self.dev, &dir_tx, &chan, &enabled)
        };
        
        if res >= LMS_SUCCESS {
            Ok(())
        }
        else {
            return Err(());
        }
    }

    fn setSampleRate(&self, rate: f64, oversample: usize) -> Result<(),()> {
        let res = unsafe {
            LMS_SetSampleRate(self.dev, &rate, oversample)
        };
        if res >= LMS_SUCCESS {
            Ok(())
        }
        else {
            return Err(());
        }
    }
    fn getSampleRate (&self, dir_tx: bool, chan: usize) -> Result<SampleRate, ()> {
        let mut host_Hz = 0.0;
        let mut rf_Hz: f64 = 0.0;
        let res: i32 = unsafe {
            LMS_GetSampleRate(self.dev, &dir_tx, &chan, &mut host_Hz as *mut f64, &mut rf_Hz as *mut f64)
        };

        if res >= LMS_SUCCESS {
            Ok(SampleRate {
                host_Hz: host_Hz,
                rf_Hz: rf_Hz
            })
        }
        else {
            return Err(());
        }
    }

    fn getSampleRateRange(&self, dir_tx: bool) -> Result<Range, ()> {
        let mut lms_range: lms_range_t = lms_range_t {
            min: 0.0,
            max: 0.0,
            step:0.0
        };
        let res = unsafe {
            LMS_GetSampleRateRange(self.dev, &dir_tx, &mut lms_range as *mut lms_range_t)
        };

        if res >= LMS_SUCCESS {
            Ok(Range {
                min: lms_range.min,
                max: lms_range.max,
                step: lms_range.step
            })
        }
        else {
            return Err(());
        }
    }

    fn setLOFrequency(&self, dir_tx: bool, chan: usize, frequency: f64) -> Result<(),()>{
        let res = unsafe {
            LMS_SetLOFrequency(self.dev, &dir_tx, &chan, &frequency)
        };
        if res >= LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }



}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            LMS_Close(self.dev);
        }

    }
}

pub struct SampleRate {
    host_Hz: f64,
    rf_Hz: f64
}

pub struct Range {
    pub min: f64,
    pub max: f64,
    pub step: f64
}