#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use libc::{c_void, c_int, c_char, c_float, c_double, size_t};

pub type lms_device_t = c_void;
pub type lms_info_str_t = [c_char;256];
pub type lms_name_t = [c_char;16];
    //fn typedef bool (*lms_prog_callback_t)(int bsent, int btotal, const char* progressMsg);

//bool???
pub const LMS_SUCCESS: c_int = 0;
pub const LMS_CH_TX: bool = true;
pub const LMS_CH_RX: bool = false;
pub const LMS_NCO_VAL_COUNT: usize = 16;

pub const BOARD_PARAM_DAC: u8 = 0;
pub const BOARD_PARAM_TEMP: u8 = 1;

pub const LMS_CLOCK_REF:size_t  =  0x0000;
///RX LO clock
pub const LMS_CLOCK_SXR:size_t  =  0x0001;
///TX LO clock
pub const LMS_CLOCK_SXT:size_t  =  0x0002;
///CGEN clock
pub const LMS_CLOCK_CGEN:size_t =  0x0003;
///RXTSP reference clock (read-only)
pub const LMS_CLOCK_RXTSP:size_t = 0x0004;
///TXTSP reference clock (read-only)
pub const LMS_CLOCK_TXTSP:size_t = 0x0005;
pub const LMS_CLOCK_EXTREF:size_t = 0x0006;
pub const LMS_ALIGN_CH_PHASE:size_t = 1<<16;

pub const LMS_LOG_CRITICAL:i32 =0;

pub const LMS_LOG_ERROR:i32    =1;

pub const LMS_LOG_WARNING:i32  =2;

pub const LMS_LOG_INFO:i32     =3;

pub const LMS_LOG_DEBUG:i32   = 4;



#[link(name = "LimeSuite")]
extern "C" {
    pub fn LMS_GetDeviceList(dev_list: *mut lms_info_str_t) -> c_int;
    pub fn LMS_Open(device: *mut *mut lms_device_t, info: *const lms_info_str_t, args: *mut c_void) -> c_int;
    pub fn LMS_Close(device: *mut lms_device_t) -> c_int;
    pub fn LMS_Init(device: *mut lms_device_t) -> c_int;
    pub fn LMS_GetNumChannels(device: *mut lms_device_t, dir_tx: bool ) -> c_int;
    pub fn LMS_EnableChannel(device: *mut lms_device_t, dir_tx: bool,
            chan: size_t , enabled: bool)->c_int;
    pub fn LMS_SetSampleRate(device: *mut lms_device_t, rate: c_double,
            oversample: size_t ) -> c_int;
    pub fn LMS_GetSampleRate(device: *mut lms_device_t, dir_tx: bool,
            chan: size_t , host_Hz: *mut c_double, rf_Hz: *mut c_double) -> c_int;
    pub fn LMS_GetSampleRateRange(device: *mut lms_device_t, dir_tx: bool,
            range: *mut lms_range_t) -> c_int;
    pub fn LMS_SetLOFrequency(device: *mut lms_device_t, dir_tx: bool,
            chan: size_t, frequency: c_double) -> c_int;
    pub fn LMS_GetLOFrequency(device: *mut lms_device_t, dir_tx: bool,
            chan: size_t, frequency: *mut c_double)-> c_int;
    pub fn LMS_GetLOFrequencyRange(device: *mut lms_device_t, dir_tx: bool,
            range: *mut lms_range_t)-> c_int;
    pub fn LMS_GetAntennaList(device: *mut lms_device_t, dir_tx: bool,
            chan: size_t, list: *mut lms_name_t) -> c_int;
    pub fn LMS_SetAntenna(device: *mut lms_device_t, dir_tx: bool,
            chan: size_t, index: size_t ) -> c_int;
    pub fn  LMS_GetAntenna(device: *mut lms_device_t, dir_tx: bool,
            chan: size_t ) -> c_int;
    pub fn  LMS_GetAntennaBW(device: *mut lms_device_t, dir_tx: bool,
            chan: size_t , index: size_t , range: *mut lms_range_t) -> c_int;
    pub fn  LMS_SetNormalizedGain(device: *mut lms_device_t, dir_tx: bool,
            chan: size_t ,gain: c_double ) -> c_int;
    pub fn LMS_SetGaindB(device: *mut lms_device_t, dir_tx: bool,
            chan: size_t, gain: u32) -> c_int;
    pub fn LMS_GetNormalizedGain(device: *mut lms_device_t, dir_tx: bool,
            chan: size_t, gain: *mut c_double) -> c_int;
    pub fn LMS_GetGaindB(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, gain: *mut u32)-> c_int;
    pub fn LMS_SetLPFBW(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, bandwidth: c_double)-> c_int;
    pub fn LMS_GetLPFBW(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, bandwidth: *mut c_double)-> c_int;

    pub fn LMS_GetLPFBWRange(device: *mut lms_device_t, dir_tx: bool,
        range: *mut lms_range_t)-> c_int;
    pub fn LMS_SetLPF(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, enable: bool) -> c_int;
    pub fn LMS_SetGFIRLPF(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, enabled: bool, bandwidth: c_double) -> c_int;
    pub fn LMS_Calibrate(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, bw: c_double , flags: u32) -> c_int;
    pub fn LMS_LoadConfig(device: *mut lms_device_t, filename: *const c_char) -> c_int;
    pub fn LMS_SaveConfig(device: *mut lms_device_t, filename: *const c_char) -> c_int;
    pub fn LMS_SetTestSignal(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, sig: &lms_testsig_t , dc_i: i16 , dc_q: i16 ) -> c_int;
    pub fn LMS_GetTestSignal(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, sig: *mut lms_testsig_t) -> c_int;
    pub fn LMS_GetChipTemperature(dev: *mut lms_device_t, ind: size_t,
        temp: *mut c_double) -> c_int;
    pub fn LMS_SetSampleRateDir(device: *mut lms_device_t, dir_tx: bool,
        rate: c_double, oversample: size_t) -> c_int;
    pub fn LMS_SetNCOFrequency(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, freq: *const c_double, pho: c_double) -> c_int;
    pub fn LMS_GetNCOFrequency(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, freq: *mut c_double, pho: *mut c_double)-> c_int;
    pub fn LMS_SetNCOPhase(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, phases: *const c_double, fcw: c_double) -> c_int;
    pub fn LMS_GetNCOPhase(ldevice: *mut lms_device_t, dir_tx: bool,
        chan: size_t, phases: *mut c_double, fcw: *mut c_double)->c_int;
    pub fn LMS_SetNCOIndex(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, index: i32 , downconv: bool ) -> c_int;
    pub fn LMS_GetNCOIndex(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t ) -> c_int;
    pub fn LMS_SetGFIRCoeff(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, filt: lms_gfir_t , coef: *const c_double, count: size_t) -> c_int;
    pub fn LMS_GetGFIRCoeff(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, filt: lms_gfir_t , coef: *const c_double) -> c_int;
    pub fn LMS_SetGFIR(device: *mut lms_device_t, dir_tx: bool,
        chan: size_t, filt: lms_gfir_t , enabled: bool) -> c_int;
    pub fn LMS_EnableCache(device: *mut lms_device_t, enable: bool )-> c_int;
    pub fn LMS_Reset(device: *mut lms_device_t) -> c_int;
    pub fn LMS_ReadLMSReg(device: *mut lms_device_t, address: u32 ,
        val: *mut u16) -> c_int;
    pub fn LMS_WriteLMSReg(device: *mut lms_device_t, address: u32,
        val: u16);
    //pub fn LMS_ReadParam(device: *mut lms_device_t,
    //    param: &LMS7Parameter , val: *mut u16);
    //pub fn LMS_WriteParam(device: *mut lms_device_t,
    //    param: &LMS7Parameter , val: &u16 ) -> c_int;
    pub fn LMS_ReadFPGAReg(device: *mut lms_device_t, address: u32,
        val: *mut u16) -> c_int;
    pub fn LMS_WriteFPGAReg(device: *mut lms_device_t, address: u32,
        val: *mut u16) -> c_int;
    pub fn LMS_ReadCustomBoardParam(device: *mut lms_device_t,
        id: u8 , val: *mut c_double, units: *mut lms_name_t) -> c_int;
    pub fn LMS_WriteCustomBoardParam(device: *mut lms_device_t,
        id: u8 , val: c_double , units: *const lms_name_t ) -> c_int;
    pub fn LMS_GetClockFreq(device: *mut lms_device_t, clk_id: size_t,
        freq: *mut c_double) -> c_int;
    pub fn LMS_SetClockFreq(device: *mut lms_device_t, clk_id: size_t,
        freq: c_double) -> c_int;
    pub fn LMS_VCTCXOWrite(dev: *mut lms_device_t , val: u16 ) -> c_int;
    pub fn LMS_VCTCXORead(dev: *mut lms_device_t, val: *mut u16)-> c_int;
    pub fn LMS_Synchronize(dev: *mut lms_device_t, toChip: bool ) -> c_int;
    pub fn LMS_GPIORead(dev: *mut lms_device_t, buffer: *mut u8 , len: size_t) -> c_int;
    pub fn LMS_GPIOWrite(dev: *mut lms_device_t, buffer: *const u8, len: size_t)-> c_int;
    pub fn LMS_GPIODirRead(dev: *mut lms_device_t, buffer: *mut u8 , len: size_t);
    pub fn LMS_GPIODirWrite(dev: *mut lms_device_t, buffer: *const u8 , len: size_t);
    pub fn LMS_SetupStream(dev: *mut lms_device_t, stream: *mut lms_stream_t) -> c_int;
    pub fn LMS_DestroyStream(dev: *mut lms_device_t, stream: *mut lms_stream_t) -> c_int;
    pub fn LMS_StartStream(stream: *mut lms_stream_t) -> c_int;
    pub fn LMS_StopStream(stream: *mut lms_stream_t) -> c_int;
    pub fn LMS_RecvStream(stream: *mut lms_stream_t, samples: *mut c_void,
        sample_count: size_t , meta: *mut lms_stream_meta_t, timeout_ms: u32 ) -> c_int;
        pub fn LMS_GetStreamStatus(stream: *mut lms_stream_t, status: *mut lms_stream_status_t)->c_int;
    pub fn LMS_SendStream(stream: *mut lms_stream_t,
        samples: *const c_void,sample_count: size_t,
        meta: *const lms_stream_meta_t, timeout_ms: u32)->c_int;
        pub fn LMS_UploadWFM(device: *mut lms_device_t, samples: *const *const c_void,
        chCount: u8 , sample_count: size_t , format: i32 )->c_int;
        pub fn LMS_EnableTxWFM(device: *mut lms_device_t, chan: u32, active: bool )->c_int;

    pub fn LMS_GetProgramModes(device: *mut lms_device_t, list: *mut lms_name_t);

    //pub fn LMS_Program(device: *mut lms_device_t, data: *const c_char,
    //    size: &size_t , mode: *const lms_name_t , callback: lms_prog_callback_t);
    pub fn  LMS_GetDeviceInfo(device: *mut lms_device_t)->*const lms_dev_info_t;
    pub fn  LMS_GetLibraryVersion()-> *const c_char;

    pub fn LMS_RegisterLogHandler(handler: extern fn (lvl: i32 , msg: *const c_char)->c_void ) -> c_void;

}

#[repr(C)]
pub struct lms_range_t {
    pub min: c_double,
    pub max: c_double,
    pub step: c_double
}

#[repr(C)]
pub enum lms_testsig_t {
    LMS_TESTSIG_NONE = 0,
    LMS_TESTSIG_NCODIV8,
    LMS_TESTSIG_NCODIV4,
    LMS_TESTSIG_NCODIV8F,
    LMS_TESTSIG_NCODIV4F,
    LMS_TESTSIG_DC       
}
/*
#[repr(C)]
pub enum foo
{
    LMS_PATH_NONE = 0, ///<No active path (RX or TX)
    LMS_PATH_LNAH = 1, ///<RX LNA_H port
    LMS_PATH_LNAL = 2, ///<RX LNA_L port
    LMS_PATH_LNAW = 3, ///<RX LNA_W port
    LMS_PATH_TX1 = 1,  ///<TX port 1
    LMS_PATH_TX2 = 2,   ///<TX port 2
    LMS_PATH_AUTO = 255, ///<Automatically select port (if supported)
};
*/
#[repr(C)]
pub enum lms_gfir_t
{
    LMS_GFIR1 = 0,
    LMS_GFIR2,
    LMS_GFIR3
}



#[repr(C)]
pub struct lms_stream_meta_t {
    timestamp: u64,
    waitForTimestamp: bool,
    flushPartialPacket: bool
}

#[repr(C)] 
pub enum dataFmt_t {//Esto dentro de lms_stream_t
    LMS_FMT_F32=0,    
    LMS_FMT_I16,      
    LMS_FMT_I12       
}
#[repr(C)]
pub enum linkFmt_t { //Esto dentro de lms_stream_t
    LMS_LINK_FMT_DEFAULT=0,             
    LMS_LINK_FMT_I16,   
    LMS_LINK_FMT_I12    
}
#[repr(C)]
pub struct lms_stream_t {
    handle: size_t,
    isTx: bool,
    channel: u32,
    fifoSize: u32,
    throughputVsLatency: c_float,
    dataFmt: dataFmt_t,
    linkFmt: linkFmt_t
}
#[repr(C)]
pub struct lms_stream_status_t {
    active: bool,
    fifoFilledCount: u32,
    fifoSize: u32,
    underrun: u32,
    overrun: u32,
    drppedPackets: u32,
    sampleRate: c_double,
    linkRate: c_double,
    timestamp: u64
}

#[repr(C)]
pub struct lms_dev_info_t {
    deviceName: [c_char;32],
    expensionName: [c_char;32],
    firmwareVersion: [c_char;16],
    hardwareVersion: [c_char;16],
    protocolVersion: [c_char;16],
    boardSerialNumber: u64,
    gatewareVersion: [c_char;16],
    gatewareTargetBoard: [c_char;32],
}
