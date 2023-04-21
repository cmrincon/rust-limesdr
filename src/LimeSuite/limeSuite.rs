#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::LimeSuite::lib::{
    *,
    lms_testsig_t::LMS_TESTSIG_NONE,
    lms_gfir_t::* };
use std::ffi::{CStr, c_void};
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
                    string.as_ptr() as *const [u8;256], std::ptr::null_mut())
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
            LMS_GetNumChannels(self.dev, dir_tx)
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
            LMS_EnableChannel(self.dev, dir_tx, chan, enabled)
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
            LMS_SetSampleRate(self.dev, rate, oversample)
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
            LMS_GetSampleRate(self.dev, dir_tx, chan, &mut host_Hz as *mut f64, &mut rf_Hz as *mut f64)
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
        let mut lms_name: [lms_name_t; 6] = [[0_u8;std::mem::size_of::<lms_name_t>()]; 6];
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
//filename should have a null terminated char \0 at the end.
    pub fn LMS_LoadConfig(&self, filename: &str) -> Result<(),()> {
    //Is from_btes_with_nul the right function for this????
        let s = String::from(filename) + "\0";
        let c_filename = CStr::from_bytes_with_nul(s.as_bytes());
        if let Err(_) = c_filename  {
            return Err(());
        }
        let c_filename = c_filename.unwrap();

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
//filename should have a null terminated char \0 at the end.
    pub fn LMS_SaveConfig(&self, filename: &str) -> Result<(),()> {
            let c_filename =  CStr::from_bytes_with_nul(filename.as_bytes());
            if let Err(_) = c_filename  {
                return Err(());
            }
            let c_filename = c_filename.unwrap();

        let res = unsafe {
            LMS_SaveConfig(self.dev, c_filename.as_ptr())
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn  LMS_SetTestSignal(&self, dir_tx: bool, chan: usize, testsig: lms_testsig_t, dc_i: i16, dc_q: i16) -> Result<(),()> {

        let res = unsafe {
            LMS_SetTestSignal(self.dev, dir_tx, chan, &testsig, dc_i, dc_q)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GetTestSignal(&self, dir_tx: bool, chan: usize) -> Result<lms_testsig_t,()> {

        let mut lms_testsig: lms_testsig_t = LMS_TESTSIG_NONE;

        let res = unsafe {
            LMS_GetTestSignal(self.dev, dir_tx, chan, &mut lms_testsig)
        };

        if res == LMS_SUCCESS {
            Ok(lms_testsig)
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GetChipTemperature(&self, ind: usize) -> Result<f64,()> {

        let mut temp: f64 = 0.0;

        let res = unsafe {
            LMS_GetChipTemperature(self.dev, ind, &mut temp)
        };

        if res == LMS_SUCCESS {
            Ok(temp)
        }
        else {
            Err(())
        }
    }

    pub fn LMS_SetSampleRateDir(&self, dir_tx: bool, rate: f64, oversample: usize) -> Result<(),()> {

        let res = unsafe {
            LMS_SetSampleRateDir(self.dev, dir_tx, rate, oversample)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_SetNCOFrequency(&self, dir_tx: bool, chan: usize, freq: &[f64;LMS_NCO_VAL_COUNT], pho: f64) -> Result<(),()> {

        let res = unsafe {
            LMS_SetNCOFrequency(self.dev, dir_tx, chan, freq.as_ptr(), pho)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GetNCOFrequency(&self, dir_tx:bool, chan: usize) -> Result<([f64;LMS_NCO_VAL_COUNT], f64),()> {

        let mut pho: f64 = 0.0;
        let mut freq: [f64; LMS_NCO_VAL_COUNT] = [0.0;LMS_NCO_VAL_COUNT];
        let res = unsafe {
            LMS_GetNCOFrequency(self.dev, dir_tx, chan, freq.as_mut_slice().as_mut_ptr(), &mut pho)
        };

        if res == LMS_SUCCESS {
            Ok((freq, pho))
        }
        else {
            Err(())
        }
    }

    pub fn LMS_SetNCOPhase(&self, dir_tx: bool, chan: usize, phases: &[f64; LMS_NCO_VAL_COUNT], fcw: f64) -> Result<(),()> {

        let res = unsafe {
            LMS_SetNCOPhase(self.dev, dir_tx, chan, phases.as_slice().as_ptr(), fcw)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GetNCOPhase(&self, dir_tx: bool, chan: usize) -> Result<([f64;LMS_NCO_VAL_COUNT], f64), ()> {
        let mut fcw: f64 = 0.0;
        let mut phases: [f64;LMS_NCO_VAL_COUNT] = [0.0; LMS_NCO_VAL_COUNT];

        let res = unsafe {
            LMS_GetNCOPhase(self.dev, dir_tx, chan, phases.as_mut_slice().as_mut_ptr(), &mut fcw)
        };

        if res == LMS_SUCCESS {
            Ok((phases, fcw))
        }
        else {
            Err(())
        }
    }

    pub fn LMS_SetNCOIndex(&self, dir_tx:bool, chan: usize, index: i32, downconv: bool) -> Result<(),()> {

        let res = unsafe {
            LMS_SetNCOIndex(self.dev, dir_tx, chan, index, downconv)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }
//Is u8 enough for index???
    pub fn LMS_GetNCOIndex(&self, dir_tx: bool, chan: usize) -> Result<u8, ()> {

        let res = unsafe {
            LMS_GetNCOIndex(self.dev, dir_tx, chan)
        };

        if res >= 0 {
            Ok(res as u8)
        }
        else {
            Err(())
        }
    }

    pub fn LMS_SetGFIRCoeff(&self, dir_tx: bool, chan: usize, filt: lms_gfir_t, coef: Vec::<f64>, count: usize) -> Result<(),()> {

        let res = unsafe {
            LMS_SetGFIRCoeff(self.dev, dir_tx, chan, filt, coef.as_ptr(), count)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GetGFIRCoeff(&self, dir_tx: bool, chan: usize, filt: lms_gfir_t) -> Result<Vec::<f64>, ()> {

        let vec_size: usize = match filt{
            LMS_GFIR1 => 40,
            LMS_GFIR2 => 40,
            LMS_GFIR3 => 120
        };
        let mut ret_vec: Vec::<f64> = vec![0.0; vec_size];
        let res = unsafe {
            LMS_GetGFIRCoeff(self.dev, dir_tx, chan, filt, ret_vec.as_mut_ptr())
        };

        if res == LMS_SUCCESS {
            return Ok(ret_vec);
        }
        else {
            return Err(());
        }
    }

    pub fn LMS_SetGFIR(&self, dir_tx: bool, chan: usize, filt: lms_gfir_t, enabled: bool) -> Result<(),()> {

        let res = unsafe {
            LMS_SetGFIR(self.dev, dir_tx, chan, filt, enabled)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_EnableCache(&self, enable: bool) -> Result<(),()> {
        let res = unsafe {
            LMS_EnableCache(self.dev, enable)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_Reset(&self) -> Result<(),()> {
        let res = unsafe {
            LMS_Reset(self.dev)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_ReadLMSReg(&self, address: u32) -> Result<u16,()> {
        let mut ret: u16 = 0_u16;
        let res = unsafe {
            LMS_ReadLMSReg(self.dev, address, &mut ret)
        };

        if res == LMS_SUCCESS {

            Ok(ret)
        }
        else {
            Err(())
        }
    }

    pub fn LMS_WriteLMSReg(&self, address: u32, val: u16) -> Result<(),()> {

        let res = unsafe {
            LMS_WriteLMSReg(self.dev, address, val)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
}

    pub fn LMS_ReadFPGAReg(&self, address: u32) -> Result<u16,()> {
        let mut val: u16 = 0_u16;
        let res = unsafe {
            LMS_ReadFPGAReg(self.dev, address, &mut val)
        };

        if res == LMS_SUCCESS {
            Ok(val)
        }

        else {
            Err(())
        }
}

    pub fn LMS_WriteFPGAReg(&self, address: u32, val: u16) -> Result<(),()> {

        let res = unsafe {
            LMS_WriteFPGAReg(self.dev, address, val)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }

        else {
            Err(())
        }

    }
    /* NOT IMPLEMENTED YET */
    //pub fn ReadCustomBoardParam(&self, id: u8) -> Result((f64, 

    //pub fn WriteCustomBoardParam(&self, id: u8, val f64 ...) -> 
    
    pub fn LMS_GetClockFreq(&self, clk_id: usize) -> Result<f64,()> {
        let mut ret: f64 = 0.0;
        let res = unsafe {
            LMS_GetClockFreq(self.dev, clk_id, &mut ret)
        };

        if res == LMS_SUCCESS {
            Ok(ret)
        }

        else {
            Err(())
        }
    }

    pub fn LMS_SetCLockFreq(&self, clk_id: usize, freq: f64) -> Result<(),()> {

        let res = unsafe {
            LMS_SetClockFreq(self.dev, clk_id, freq)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_VCTCXOWrite(&self, val: u16) -> Result<(),()> {
        let res = unsafe {
            LMS_VCTCXOWrite(self.dev, val)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }
    
    pub fn LMS_VCTCXORead(&self) -> Result<u16,()> {
        let mut val: u16 = 0;
        let res = unsafe {
            LMS_VCTCXORead(self.dev, &mut val)
        };

        if res == LMS_SUCCESS {
                Ok(val)
        }
        else {
            Err(())
        }
    }


    pub fn LMS_Synchronize(&self, toChip: bool) -> Result<(),()> {

        let res = unsafe {
            LMS_Synchronize(self.dev, toChip)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GPIORead(&self, len: usize) -> Result<Vec<u8>,()> {

        let mut buffer = Vec::<u8>::with_capacity(len);
        let res = unsafe {
            LMS_GPIORead(self.dev, buffer.as_mut_ptr(),buffer.capacity())
        };

        if res == LMS_SUCCESS {
            Ok(buffer)
        }

        else {
            Err(())
        }
    }
    pub fn LMS_GPIOWrite(&self, buffer: &mut [u8]) -> Result<(),()> {
        let len = buffer.len();

        let res = unsafe {
            LMS_GPIOWrite(self.dev, buffer.as_mut_ptr(), len)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GPIODirRead(&self, len: usize) -> Result<Vec<u8>,()> {
        let mut buffer = Vec::<u8>::with_capacity(len);

        let res = unsafe {
            LMS_GPIODirRead(self.dev, buffer.as_mut_ptr(), buffer.capacity())
        };
        if res == LMS_SUCCESS {
            Ok(buffer)
        }
        else {
            Err(())
        }
    }

    pub fn LMS_GPIODirWrite(&self, buffer: &mut [u8]) -> Result<(),()> {
        let len = buffer.len(); 
        let res = unsafe { 
            LMS_GPIODirWrite(self.dev, buffer.as_ptr(), len)
        };
        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_SetupStream(&self, stream: &mut lms_stream_t) -> Result<(),()> {
        let res = unsafe {
            LMS_SetupStream(self.dev, stream)
        };
        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_DestroyStream(&self, stream: &mut lms_stream_t) -> Result<(),()> {
        let res = unsafe {
            LMS_DestroyStream(self.dev, stream)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_StartStream(stream: &mut lms_stream_t) -> Result<(),()> {
        let res = unsafe {
            LMS_StartStream(stream)
        };
        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_StopStream(stream: &mut lms_stream_t) -> Result<(),()> {
        let res = unsafe {
            LMS_StopStream(stream)
        };
        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_RecvStream(stream: &mut lms_stream_t, sample_count: usize, timeout_ms: u32) -> Result<(Vec<u8>,lms_stream_meta_t),()> {

        let mut buffer = Vec::<u8>::with_capacity(sample_count);
        let mut meta = lms_stream_meta_t {
            timestamp: 0, 
            waitForTimestamp: false,
            flushPartialPacket: false
        };

        let res = unsafe {
            LMS_RecvStream(stream, buffer.as_mut_ptr() as *mut c_void, buffer.capacity(), &mut meta, timeout_ms)
        };
        if res == LMS_SUCCESS {
            Ok((buffer,meta))
        }
        else {
            Err(())
        }
    }
    pub fn LMS_GetStreamStatus(stream: &mut lms_stream_t) -> Result<lms_stream_status_t,()> {
        
        let mut status = lms_stream_status_t {
            active: false,
            fifoFilledCount: 0,
            fifoSize: 0,
            underrun: 0,
            overrun: 0,
            dropedPackets: 0,
            sampleRate: 0.0,
            linkRate: 0.0,
            timestamp: 0
        };
        let res = unsafe {
            LMS_GetStreamStatus(stream, &mut status)
        };

        if res == LMS_SUCCESS {
            Ok(status)
        }

        else {
            Err(())
        }
    }

    pub fn LMS_SendStream(stream: &mut lms_stream_t, buffer: &[u8], meta: &lms_stream_meta_t, timeout_ms: u32) -> Result<u32,()> {

        let len = buffer.len();
        let buff_void = buffer.as_ptr() as *const c_void;
        
        let res = unsafe {
            LMS_SendStream(stream, buff_void, len, meta, timeout_ms)
        };
        if res >= 0 {
            Ok(res.try_into().unwrap())
        }
        else {
            Err(())
        }
    }

    pub fn LMS_UploadWFM(&self, samples: & &[u8], format: i32) -> Result<(),()> {
    let chCount = samples.len() as u8;
    let sample_count = (*samples).len();

    let samples_void = samples.as_ptr() as *const *const c_void;

    let res = unsafe {
        LMS_UploadWFM(self.dev, samples_void, chCount, sample_count, format)
    };

    if res == LMS_SUCCESS {
        Ok(())
    }

    else {
        Err(())
    }
    }

    pub fn LMS_EnableTxWFM(&self, chan: u32, active: bool) -> Result<(),()> {
        let res = unsafe {
            LMS_EnableTxWFM(self.dev, chan, active)
        };

        if res == LMS_SUCCESS {
            Ok(())
        }
        else {
            Err(())
        }
    }
    //NOT IMPLEMENTED YET
    //pub fn LMS_GetProgramModes(&self) -> Result<&str,()> {
     //   str
    //pub fn LMS_Program(&self, data: 
    
    //pub fn LMS_GetLibraryVersion() -> &str {
    //""
    //}

    //pub fn LMS_RegisterLogHandler() ->



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
/*
pub struct testsignal {
    LMS_TESTSIG_NONE = 0,
    LMS_TESTSIG_NCODIV8,
    LMS_TESTSIG_NCODIV4,
    LMS_TESTSIG_NCODIV8F,
    LMS_TESTSIG_NCODIV4F,
    LMS_TESTSIG_DC
}
*/
