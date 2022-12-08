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

pub struct Device {
    dev: *mut lms_device_t,
}

impl Device {
    pub fn open(dev_info: &str ) -> Result<Device,()> {
        let mut dev: *mut lms_device_t = std::ptr::null_mut();
        let string: std::ffi::CString = 
            std::ffi::CString::new(dev_info).expect("String failed");
        match unsafe {
            LMS_Open(std::ptr::addr_of_mut!(dev),
                    string.as_ptr() as *const [i8;256], std::ptr::null_mut())
        } {
            LMS_SUCCESS => Ok(Device {
                dev: dev 
            }),
            _ => return Err(()),
        } 
    }

    pub fn init(&self) -> Result<(), ()> {
        match unsafe {
            LMS_Init(self.dev)} {
                LMS_SUCCESS => Ok(()),
                _ => return Err(()),
            }
    }

    pub fn getNumChannels(&self, dir_tx: bool) -> Result<i32,()> {
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

    pub fn LMS_EnableChannel(&self, dir_tx: bool,
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

    pub fn setSampleRate(&self, rate: f64, oversample: usize) -> Result<(),()> {
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
    pub fn getSampleRate (&self, dir_tx: bool, chan: usize) -> Result<SampleRate, ()> {
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

    pub fn getSampleRateRange(&self, dir_tx: bool) -> Result<Range, ()> {
        let mut lms_range: lms_range_t = lms_range_t {
            min: 0.0,
            max: 0.0,
            step:0.0
        };
        let res = unsafe {
            LMS_GetSampleRateRange(self.dev, dir_tx, &mut lms_range as *mut lms_range_t)
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

    pub fn setLOFrequency(&self, dir_tx: bool, chan: usize, frequency: f64) -> Result<(),()>{
        let res = unsafe {
            LMS_SetLOFrequency(self.dev, dir_tx, chan, frequency)
        };
        if res >= LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GetLOFrequency(&self, dir_tx: bool, chan: usize) -> Result<f64,()> {
        let mut freq:f64 = 0.0;
        let res =  unsafe {
            LMS_GetLOFrequency(self.dev, dir_tx, chan, &mut freq as *mut f64)
        };
        if res == LMS_SUCCESS {
            Ok(freq)
        }
        else {
            Err(())
        }

    }

    pub fn LMS_GetLOFrequencyRange(&self, dir_tx: bool) -> Result<Range,()> {

        let mut lms_range: lms_range_t = lms_range_t {
            min: 0.0,
            max: 0.0,
            step:0.0
        };

        let res = unsafe {
            LMS_GetLOFrequencyRange(self.dev, dir_tx, &mut lms_range as  *mut lms_range_t)
        };
        if res == LMS_SUCCESS {
            Ok(Range {
                min: lms_range.min,
                max: lms_range.max,
                step: lms_range.step
            })
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GetAntennaList(&self, dir_tx: bool, chan: usize) -> Result<std::vec::Vec<&str>,()> {

    //It may return Box<str> in this function?
        let mut lms_name: [lms_name_t; 6] = [[0_i8;std::mem::size_of::<lms_name_t>()]; 6];
        let ptr_lms_name: *mut lms_name_t = &mut lms_name[0];

        let res = unsafe {
            LMS_GetAntennaList(self.dev, dir_tx, chan, ptr_lms_name)
        };

        if res > 0 && ptr_lms_name != std::ptr::null_mut() {
            let n_antennas = res as usize;
            let mut ret =
                std::vec::Vec::<&str>::with_capacity(n_antennas);
            for name in &lms_name[0..n_antennas - 1] {
                let c_name = unsafe { CStr::from_ptr(&name[0])};
                ret.push(c_name.to_str().unwrap());
            }
            Ok(ret)
        }
        else {
            Err(())
        }
    }

    pub fn LMS_SetAntenna(&self, dir_tx: bool, chan: usize, index: usize) -> Result<(),()> {

        let res = unsafe {
            LMS_SetAntenna(self.dev, dir_tx, chan, index)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GetAntenna(&self, dir_tx: bool, chan: usize) -> Result<u8, ()> {
        let res = unsafe{
            LMS_GetAntenna(self.dev, dir_tx, chan)
        };
        if res >= 0 {
            Ok(res as u8)
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GetAntennaBW(&self, dir_tx:bool, chan: usize, index: usize) -> Result<Range,()> {

        let mut lms_range: lms_range_t = lms_range_t {
            min: 0.0,
            max: 0.0,
            step:0.0
        };
        let res = unsafe {
            LMS_GetAntennaBW(self.dev, dir_tx, chan, index, &mut lms_range)
        };

        if res == LMS_SUCCESS {
            Ok(Range {
                min: lms_range.min,
                max: lms_range.max,
                step: lms_range.step
            })
        }
        else {
            Err(())
        }
    }

    pub fn LMS_SetNormalizedGain(&self, dir_tx: bool, chan: usize, gain: f64) -> Result<(),()> {

        let res = unsafe {
            LMS_SetNormalizedGain(self.dev, dir_tx, chan, gain)
        };
        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_SetGaindB(&self, dir_tx: bool, chan: usize, gain: u32) -> Result<(),()> {
        let res = unsafe {
            LMS_SetGaindB(self.dev, dir_tx, chan, gain)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GetNormalizedGain(&self, dir_tx: bool, chan: usize) -> Result<f64,()> {
        let mut gain: f64 = 0.0;
        let res = unsafe {
            LMS_GetNormalizedGain(self.dev, dir_tx, chan, &mut gain)
        };

        if res == LMS_SUCCESS {
            Ok(gain)
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GetGaindB(&self, dir_tx: bool, chan: usize) -> Result<u32,()> {
        let mut gain: u32 = 0;
        let res = unsafe {
            LMS_GetGaindB(self.dev, dir_tx, chan, &mut gain)
        };

        if res == LMS_SUCCESS {
            Ok(gain)
        }
        else {
            Err(())
        }
    }

    pub fn LMS_SetLPFBW(&self, dir_tx: bool, chan: usize, bandwidth: f64) -> Result<(),()> {
        let res = unsafe {
            LMS_SetLPFBW(self.dev, dir_tx, chan, bandwidth)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GetLPFBW(&self, dir_tx: bool, chan: usize) -> Result<f64, ()> {
        let mut bandwidth: f64 = 0.0;
        let res = unsafe {
            LMS_GetLPFBW(self.dev, dir_tx, chan, &mut bandwidth)
        };

        if res == LMS_SUCCESS {
            Ok(bandwidth)
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GetLPFBWRange(&self, dir_tx: bool) -> Result<Range,()> {
        let mut lms_range: lms_range_t = lms_range_t {
            min: 0.0,
            max: 0.0,
            step:0.0
        };
        let res = unsafe {
            LMS_GetLPFBWRange(self.dev, dir_tx, &mut lms_range)
        };

        if res == LMS_SUCCESS {
            Ok(Range {
                min: lms_range.min,
                max: lms_range.max,
                step: lms_range.step
            })
        }
        else {
            Err(())
        }
    }

    pub fn LMS_SetLPF(&self, dir_tx: bool, chan: usize, enable: bool) -> Result<(),()> {
        let res = unsafe {
            LMS_SetLPF(self.dev, dir_tx, chan, enable)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_SetGFIRLPF(&self, dir_tx: bool, chan: usize, enabled: bool, bandwidth: f64) -> Result<(),()> {
        let res = unsafe {
            LMS_SetGFIRLPF(self.dev, dir_tx, chan, enabled, bandwidth)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_Calibrate(&self, dir_tx: bool, chan: usize, bw: f64, flags: u32) -> Result<(),()> {
        let res = unsafe {
            LMS_Calibrate(self.dev, dir_tx, chan, bw, flags)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_LoadConfig(&self, filename: &str) -> Result<(),()> {
    //Is from_btes_with_nul the right function for this????
        let c_filename: &CStr = CStr::from_bytes_with_nul(filename.as_bytes()).unwrap();
        let res = unsafe {
            LMS_LoadConfig(self.dev, c_filename.as_ptr())
        };

        if res == LMS_SUCCESS {
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
