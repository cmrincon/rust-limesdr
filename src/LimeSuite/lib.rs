
#![allow(non_camel_case_types)]

use libc::{c_void, c_int, c_char, c_uchar, uint8_t, uint16_t, uint32_t, c_double, size_t, int16_t};

type lms_device_t = c_void;
type lms_info_str_t = [c_char;256];
type lms_name_t = [c_char;16];


//bool???
const LMS_SUCCESS: c_int = 0;
const LMS_CH_TX: bool = true;
const LMS_CH_RX: bool = false;
const LMS_NCO_VAL_COUNT: c_int = 16;

const BOARD_PARAM_DAC = 0;
const BOARD_PARAM_TEMP = 1;

const LMS_CLOCK_REF  =  0x0000
///RX LO clock
const LMS_CLOCK_SXR  =  0x0001
///TX LO clock
const LMS_CLOCK_SXT  =  0x0002
///CGEN clock
const LMS_CLOCK_CGEN =  0x0003
///RXTSP reference clock (read-only)
const LMS_CLOCK_RXTSP = 0x0004
///TXTSP reference clock (read-only)
const LMS_CLOCK_TXTSP = 0x0005
const LMS_CLOCK_EXTREF = 0x0006


#[link(name = "LimeSuite")]
extern "C" {
    fn LMS_GetDeviceList(dev_list: *mut lms_info_str_t) -> c_int;
    fn LMS_Open(device: *mut *mut lms_device_t, info: &lms_info_str_t, args: *mut c_void) -> c_int;
    fn LMS_Close(device: *mut lms_device_t) -> c_int;
    fn LMS_Init(device: *mut lms_device_t) -> c_int;
    fn LMS_GetNumChannels(device: *mut lms_device_t, dir_tx: &bool ) -> c_int;
    fn LMS_EnableChannel(device: *mut lms_device_t, dir_tx: &bool,
            chan: &size_t , enabled: &bool)->c_int;
    fn LMS_SetSampleRate(device: *mut lms_device_t, rate: &c_double,
            oversample: size_t ) -> c_int;
    fn LMS_GetSampleRate(device: *mut lms_device_t, dir_tx: &bool,
            chan: &size_t , host_Hz: *mut c_double, rf_Hz: *mut c_double) -> c_int;
    fn LMS_GetSampleRateRange(device: *mut lms_device_t, dir_tx: &bool,
            range: *mut lms_range_t) -> c_int;
    fn LMS_SetLOFrequency(device: *mut lms_device_t, dir_tx: &bool,
            chan: &size_t, frequency: &c_double)-> c_int;
    fn LMS_GetLOFrequency(device: *mut lms_device_t, dir_tx: &bool,
            chan: &size_t, frequency: *mut c_double)-> c_int;
    fn LMS_GetLOFrequencyRange(device: *mut lms_device_t, dir_tx: &bool,
            range: *mut lms_range_t)-> c_int;
    fn LMS_GetAntennaList(device: *mut lms_device_t, dir_tx: &bool,
            chan: &size_t, list: *mut lms_name_t) -> c_int;
    fn LMS_SetAntenna(device: *mut lms_device_t, dir_tx: &bool,
            chan: &size_t, index: &size_t ) -> c_int;
    fn  LMS_GetAntenna(device: *mut lms_device_t, dir_tx: &bool,
            chan: &size_t ) -> c_int;
    fn  LMS_GetAntennaBW(device: *mut lms_device_t, dir_tx: &bool,
            chan: &size_t , index: &size_t , range: *mut lms_range_t) -> c_int;
    fn  LMS_SetNormalizedGain(device: *mut lms_device_t, dir_tx: &bool,
            chan: &size_t ,gain: &c_double ) -> c_int;
    fn LMS_SetGaindB(device: *mut lms_device_t, dir_tx: &bool,
            chan: &size_t, gain: &uint32_t) -> c_int;
    fn LMS_GetNormalizedGain(device: *mut lms_device_t, dir_tx: &bool,
            chan: &size_t, gain: *mut c_double) -> c_int;
    fn LMS_GetGaindB(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, gain: *mut uint32_t)-> c_int;
    fn LMS_SetLPFBW(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, bandwidth: &c_double)-> c_int;
    fn LMS_GetLPFBW(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, bandwidth: *mut c_double)-> c_int;

    fn LMS_GetLPFBWRange(device: *mut lms_device_t, dir_tx: &bool,
        range: *mut lms_range_t)-> c_int;
    fn LMS_SetLPF(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, enable: &bool) -> c_int;
    fn LMS_SetGFIRLPF(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, enabled: &bool, bandwidth: &c_double) -> c_int;
    fn LMS_Calibrate(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, bw: &c_double , flags: &uint32_t) -> c_int;
    fn LMS_LoadConfig(device: *mut lms_device_t, filename: *const c_char) -> c_int;
    fn LMS_SaveConfig(device: *mut lms_device_t, filename: *const c_char) -> c_int;
    fn LMS_SetTestSignal(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, sig: &lms_testsig_t , dc_i: &int16_t , dc_q: &int16_t );
    fn LMS_GetTestSignal(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, sig: *mut lms_testsig_t) -> c_int;
    fn LMS_GetChipTemperature(dev: *mut lms_device_t, ind: &size_t,
        temp: *mut c_double) -> c_int;
    fn LMS_SetSampleRateDir(device: *mut lms_device_t, dir_tx: &bool,
        rate: &c_double, oversample: &size_t) -> c_int;
    fn ALMS_SetNCOFrequency(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, freq: *const c_double, pho: &c_double) -> c_int;
    fn LMS_GetNCOFrequency(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, freq: *mut c_double, pho: *mut c_double)-> c_int;
    fn LMS_SetNCOPhase(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, phases: *const float_type, fcw: &c_double) -> c_int;
    fn LMS_GetNCOPhase(ldevice: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, phases: *mut c_double, fcw: *mut c_double)->c_int;
    fn LMS_SetNCOIndex(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, index: &i32 , downconv: &bool ) -> c_int;
    fn LMS_GetNCOIndex(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t ) -> c_int;
    fn LMS_SetGFIRCoeff(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, filt: &lms_gfir_t , coef: *const c_double,count: &size_t) -> c_int;
    fn LMS_GetGFIRCoeff(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, filt: &lms_gfir_t , coef: *const c_double) -> c_int;
    fn LMS_SetGFIR(device: *mut lms_device_t, dir_tx: &bool,
        chan: &size_t, filt: &lms_gfir_t , enabled: &bool) -> c_int;
    fn LMS_EnableCache(device: *mut lms_device_t, enable: &bool )-> c_int;
    fn LMS_Reset(device: *mut lms_device_t) -> c_int;
    fn LMS_ReadLMSReg(device: *mut lms_device_t, address: &uint32_t ,
        val: *mut uint16_t) -> c_int;
    fn LMS_WriteLMSReg(device: *mut lms_device_t, address: &uint32_t,
        val: &uint16_t);
    //fn LMS_ReadParam(device: *mut lms_device_t,
    //    param: &LMS7Parameter , val: *mut uint16_t);
    //fn LMS_WriteParam(device: *mut lms_device_t,
    //    param: &LMS7Parameter , val: &uint16_t ) -> c_int;
    fn LMS_ReadFPGAReg(device: *mut lms_device_t, address: &uint32_t,
        val: *mut uint16_t) -> c_int;
    fn LMS_WriteFPGAReg(device: *mut lms_device_t, address: &uint32_t,
        val: &uint16_t) -> c_int;
    fn LMS_ReadCustomBoardParam(device: *mut lms_device_t,
        id: &uint8_t , val: *mut c_double, units: &lms_name_t) -> c_int;
    fn LMS_WriteCustomBoardParam(device: *mut lms_device_t,
        id: &uint8_t , val: &c_double , units: &lms_name_t ) -> c_int;
    fn LMS_GetClockFreq(device: *mut lms_device_t, clk_id: &size_t,
        freq *mut c_double) -> c_int;
    fn LMS_SetClockFreq(device: *mut lms_device_t, clk_id: &size_t,
        freq: &c_double) -> c_int;
    fn LMS_VCTCXOWrite(dev: *mut lms_device_t , val: &uint16_t ) -> c_int;
    fn LMS_VCTCXORead(dev: *mut lms_device_t, val: &uint16_t)-> c_int;
    fn    

}

#[repr(C)]
struct lms_range_t {
    min: c_double,
    max: c_double,
    step: c_double
}

#[repr(C)]
enum lms_testsig_t {
    LMS_TESTSIG_NONE = 0,
    LMS_TESTSIG_NCODIV8,
    LMS_TESTSIG_NCODIV4,
    LMS_TESTSIG_NCODIV8F,
    LMS_TESTSIG_NCODIV4F,
    LMS_TESTSIG_DC       
}
/*
#[repr(C)]
enum foo
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

enum lms_gfir_t
{
    LMS_GFIR1 = 0,
    LMS_GFIR2,
    LMS_GFIR3
}

