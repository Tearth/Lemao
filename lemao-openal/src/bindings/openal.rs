/* automatically generated by rust-bindgen 0.62.0 */

pub const AL_INVALID: i32 = -1;
pub const AL_NONE: u32 = 0;
pub const AL_FALSE: u32 = 0;
pub const AL_TRUE: u32 = 1;
pub const AL_SOURCE_RELATIVE: u32 = 514;
pub const AL_CONE_INNER_ANGLE: u32 = 4097;
pub const AL_CONE_OUTER_ANGLE: u32 = 4098;
pub const AL_PITCH: u32 = 4099;
pub const AL_POSITION: u32 = 4100;
pub const AL_DIRECTION: u32 = 4101;
pub const AL_VELOCITY: u32 = 4102;
pub const AL_LOOPING: u32 = 4103;
pub const AL_BUFFER: u32 = 4105;
pub const AL_GAIN: u32 = 4106;
pub const AL_MIN_GAIN: u32 = 4109;
pub const AL_MAX_GAIN: u32 = 4110;
pub const AL_ORIENTATION: u32 = 4111;
pub const AL_CHANNEL_MASK: u32 = 12288;
pub const AL_SOURCE_STATE: u32 = 4112;
pub const AL_INITIAL: u32 = 4113;
pub const AL_PLAYING: u32 = 4114;
pub const AL_PAUSED: u32 = 4115;
pub const AL_STOPPED: u32 = 4116;
pub const AL_BUFFERS_QUEUED: u32 = 4117;
pub const AL_BUFFERS_PROCESSED: u32 = 4118;
pub const AL_SEC_OFFSET: u32 = 4132;
pub const AL_SAMPLE_OFFSET: u32 = 4133;
pub const AL_BYTE_OFFSET: u32 = 4134;
pub const AL_SOURCE_TYPE: u32 = 4135;
pub const AL_STATIC: u32 = 4136;
pub const AL_STREAMING: u32 = 4137;
pub const AL_UNDETERMINED: u32 = 4144;
pub const AL_FORMAT_MONO8: u32 = 4352;
pub const AL_FORMAT_MONO16: u32 = 4353;
pub const AL_FORMAT_STEREO8: u32 = 4354;
pub const AL_FORMAT_STEREO16: u32 = 4355;
pub const AL_REFERENCE_DISTANCE: u32 = 4128;
pub const AL_ROLLOFF_FACTOR: u32 = 4129;
pub const AL_CONE_OUTER_GAIN: u32 = 4130;
pub const AL_MAX_DISTANCE: u32 = 4131;
pub const AL_FREQUENCY: u32 = 8193;
pub const AL_BITS: u32 = 8194;
pub const AL_CHANNELS: u32 = 8195;
pub const AL_SIZE: u32 = 8196;
pub const AL_UNUSED: u32 = 8208;
pub const AL_PENDING: u32 = 8209;
pub const AL_PROCESSED: u32 = 8210;
pub const AL_NO_ERROR: u32 = 0;
pub const AL_INVALID_NAME: u32 = 40961;
pub const AL_ILLEGAL_ENUM: u32 = 40962;
pub const AL_INVALID_ENUM: u32 = 40962;
pub const AL_INVALID_VALUE: u32 = 40963;
pub const AL_ILLEGAL_COMMAND: u32 = 40964;
pub const AL_INVALID_OPERATION: u32 = 40964;
pub const AL_OUT_OF_MEMORY: u32 = 40965;
pub const AL_VENDOR: u32 = 45057;
pub const AL_VERSION: u32 = 45058;
pub const AL_RENDERER: u32 = 45059;
pub const AL_EXTENSIONS: u32 = 45060;
pub const AL_DOPPLER_FACTOR: u32 = 49152;
pub const AL_DOPPLER_VELOCITY: u32 = 49153;
pub const AL_SPEED_OF_SOUND: u32 = 49155;
pub const AL_DISTANCE_MODEL: u32 = 53248;
pub const AL_INVERSE_DISTANCE: u32 = 53249;
pub const AL_INVERSE_DISTANCE_CLAMPED: u32 = 53250;
pub const AL_LINEAR_DISTANCE: u32 = 53251;
pub const AL_LINEAR_DISTANCE_CLAMPED: u32 = 53252;
pub const AL_EXPONENT_DISTANCE: u32 = 53253;
pub const AL_EXPONENT_DISTANCE_CLAMPED: u32 = 53254;
pub const ALC_INVALID: u32 = 0;
pub const ALC_VERSION_0_1: u32 = 1;
pub const ALC_FALSE: u32 = 0;
pub const ALC_TRUE: u32 = 1;
pub const ALC_FREQUENCY: u32 = 4103;
pub const ALC_REFRESH: u32 = 4104;
pub const ALC_SYNC: u32 = 4105;
pub const ALC_MONO_SOURCES: u32 = 4112;
pub const ALC_STEREO_SOURCES: u32 = 4113;
pub const ALC_NO_ERROR: u32 = 0;
pub const ALC_INVALID_DEVICE: u32 = 40961;
pub const ALC_INVALID_CONTEXT: u32 = 40962;
pub const ALC_INVALID_ENUM: u32 = 40963;
pub const ALC_INVALID_VALUE: u32 = 40964;
pub const ALC_OUT_OF_MEMORY: u32 = 40965;
pub const ALC_DEFAULT_DEVICE_SPECIFIER: u32 = 4100;
pub const ALC_DEVICE_SPECIFIER: u32 = 4101;
pub const ALC_EXTENSIONS: u32 = 4102;
pub const ALC_MAJOR_VERSION: u32 = 4096;
pub const ALC_MINOR_VERSION: u32 = 4097;
pub const ALC_ATTRIBUTES_SIZE: u32 = 4098;
pub const ALC_ALL_ATTRIBUTES: u32 = 4099;
pub const ALC_DEFAULT_ALL_DEVICES_SPECIFIER: u32 = 4114;
pub const ALC_ALL_DEVICES_SPECIFIER: u32 = 4115;
pub const ALC_CAPTURE_DEVICE_SPECIFIER: u32 = 784;
pub const ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER: u32 = 785;
pub const ALC_CAPTURE_SAMPLES: u32 = 786;
#[doc = " 8-bit boolean"]
pub type ALboolean = ::std::os::raw::c_char;
#[doc = " character"]
pub type ALchar = ::std::os::raw::c_char;
#[doc = " signed 8-bit 2's complement integer"]
pub type ALbyte = ::std::os::raw::c_char;
#[doc = " unsigned 8-bit integer"]
pub type ALubyte = ::std::os::raw::c_uchar;
#[doc = " signed 16-bit 2's complement integer"]
pub type ALshort = ::std::os::raw::c_short;
#[doc = " unsigned 16-bit integer"]
pub type ALushort = ::std::os::raw::c_ushort;
#[doc = " signed 32-bit 2's complement integer"]
pub type ALint = ::std::os::raw::c_int;
#[doc = " unsigned 32-bit integer"]
pub type ALuint = ::std::os::raw::c_uint;
#[doc = " non-negative 32-bit binary integer size"]
pub type ALsizei = ::std::os::raw::c_int;
#[doc = " enumerated 32-bit value"]
pub type ALenum = ::std::os::raw::c_int;
#[doc = " 32-bit IEEE754 floating-point"]
pub type ALfloat = f32;
#[doc = " 64-bit IEEE754 floating-point"]
pub type ALdouble = f64;
#[doc = " void type (for opaque pointers only)"]
pub type ALvoid = ::std::os::raw::c_void;
extern "C" {
    pub fn alEnable(capability: ALenum);
}
extern "C" {
    pub fn alDisable(capability: ALenum);
}
extern "C" {
    pub fn alIsEnabled(capability: ALenum) -> ALboolean;
}
extern "C" {
    pub fn alGetString(param: ALenum) -> *const ALchar;
}
extern "C" {
    pub fn alGetBooleanv(param: ALenum, data: *mut ALboolean);
}
extern "C" {
    pub fn alGetIntegerv(param: ALenum, data: *mut ALint);
}
extern "C" {
    pub fn alGetFloatv(param: ALenum, data: *mut ALfloat);
}
extern "C" {
    pub fn alGetDoublev(param: ALenum, data: *mut ALdouble);
}
extern "C" {
    pub fn alGetBoolean(param: ALenum) -> ALboolean;
}
extern "C" {
    pub fn alGetInteger(param: ALenum) -> ALint;
}
extern "C" {
    pub fn alGetFloat(param: ALenum) -> ALfloat;
}
extern "C" {
    pub fn alGetDouble(param: ALenum) -> ALdouble;
}
extern "C" {
    pub fn alGetError() -> ALenum;
}
extern "C" {
    pub fn alIsExtensionPresent(extname: *const ALchar) -> ALboolean;
}
extern "C" {
    pub fn alGetProcAddress(fname: *const ALchar) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn alGetEnumValue(ename: *const ALchar) -> ALenum;
}
extern "C" {
    pub fn alListenerf(param: ALenum, value: ALfloat);
}
extern "C" {
    pub fn alListener3f(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
}
extern "C" {
    pub fn alListenerfv(param: ALenum, values: *const ALfloat);
}
extern "C" {
    pub fn alListeneri(param: ALenum, value: ALint);
}
extern "C" {
    pub fn alListener3i(param: ALenum, value1: ALint, value2: ALint, value3: ALint);
}
extern "C" {
    pub fn alListeneriv(param: ALenum, values: *const ALint);
}
extern "C" {
    pub fn alGetListenerf(param: ALenum, value: *mut ALfloat);
}
extern "C" {
    pub fn alGetListener3f(param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
}
extern "C" {
    pub fn alGetListenerfv(param: ALenum, values: *mut ALfloat);
}
extern "C" {
    pub fn alGetListeneri(param: ALenum, value: *mut ALint);
}
extern "C" {
    pub fn alGetListener3i(param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
}
extern "C" {
    pub fn alGetListeneriv(param: ALenum, values: *mut ALint);
}
extern "C" {
    #[doc = " SOURCE"]
    #[doc = " Sources represent individual sound objects in 3D-space."]
    #[doc = " Sources take the PCM data provided in the specified Buffer,"]
    #[doc = " apply Source-specific modifications, and then"]
    #[doc = " submit them to be mixed according to spatial arrangement etc."]
    #[doc = ""]
    #[doc = " Properties include: -"]
    #[doc = ""]
    #[doc = " Gain                              AL_GAIN                 ALfloat"]
    #[doc = " Min Gain                          AL_MIN_GAIN             ALfloat"]
    #[doc = " Max Gain                          AL_MAX_GAIN             ALfloat"]
    #[doc = " Position                          AL_POSITION             ALfloat[3]"]
    #[doc = " Velocity                          AL_VELOCITY             ALfloat[3]"]
    #[doc = " Direction                         AL_DIRECTION            ALfloat[3]"]
    #[doc = " Head Relative Mode                AL_SOURCE_RELATIVE      ALint (AL_TRUE or AL_FALSE)"]
    #[doc = " Reference Distance                AL_REFERENCE_DISTANCE   ALfloat"]
    #[doc = " Max Distance                      AL_MAX_DISTANCE         ALfloat"]
    #[doc = " RollOff Factor                    AL_ROLLOFF_FACTOR       ALfloat"]
    #[doc = " Inner Angle                       AL_CONE_INNER_ANGLE     ALint or ALfloat"]
    #[doc = " Outer Angle                       AL_CONE_OUTER_ANGLE     ALint or ALfloat"]
    #[doc = " Cone Outer Gain                   AL_CONE_OUTER_GAIN      ALint or ALfloat"]
    #[doc = " Pitch                             AL_PITCH                ALfloat"]
    #[doc = " Looping                           AL_LOOPING              ALint (AL_TRUE or AL_FALSE)"]
    #[doc = " MS Offset                         AL_MSEC_OFFSET          ALint or ALfloat"]
    #[doc = " Byte Offset                       AL_BYTE_OFFSET          ALint or ALfloat"]
    #[doc = " Sample Offset                     AL_SAMPLE_OFFSET        ALint or ALfloat"]
    #[doc = " Attached Buffer                   AL_BUFFER               ALint"]
    #[doc = " State (Query only)                AL_SOURCE_STATE         ALint"]
    #[doc = " Buffers Queued (Query only)       AL_BUFFERS_QUEUED       ALint"]
    #[doc = " Buffers Processed (Query only)    AL_BUFFERS_PROCESSED    ALint"]
    pub fn alGenSources(n: ALsizei, sources: *mut ALuint);
}
extern "C" {
    pub fn alDeleteSources(n: ALsizei, sources: *const ALuint);
}
extern "C" {
    pub fn alIsSource(sid: ALuint) -> ALboolean;
}
extern "C" {
    pub fn alSourcef(sid: ALuint, param: ALenum, value: ALfloat);
}
extern "C" {
    pub fn alSource3f(sid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
}
extern "C" {
    pub fn alSourcefv(sid: ALuint, param: ALenum, values: *const ALfloat);
}
extern "C" {
    pub fn alSourcei(sid: ALuint, param: ALenum, value: ALint);
}
extern "C" {
    pub fn alSource3i(sid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
}
extern "C" {
    pub fn alSourceiv(sid: ALuint, param: ALenum, values: *const ALint);
}
extern "C" {
    pub fn alGetSourcef(sid: ALuint, param: ALenum, value: *mut ALfloat);
}
extern "C" {
    pub fn alGetSource3f(sid: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
}
extern "C" {
    pub fn alGetSourcefv(sid: ALuint, param: ALenum, values: *mut ALfloat);
}
extern "C" {
    pub fn alGetSourcei(sid: ALuint, param: ALenum, value: *mut ALint);
}
extern "C" {
    pub fn alGetSource3i(sid: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
}
extern "C" {
    pub fn alGetSourceiv(sid: ALuint, param: ALenum, values: *mut ALint);
}
extern "C" {
    pub fn alSourcePlayv(ns: ALsizei, sids: *const ALuint);
}
extern "C" {
    pub fn alSourceStopv(ns: ALsizei, sids: *const ALuint);
}
extern "C" {
    pub fn alSourceRewindv(ns: ALsizei, sids: *const ALuint);
}
extern "C" {
    pub fn alSourcePausev(ns: ALsizei, sids: *const ALuint);
}
extern "C" {
    pub fn alSourcePlay(sid: ALuint);
}
extern "C" {
    pub fn alSourceStop(sid: ALuint);
}
extern "C" {
    pub fn alSourceRewind(sid: ALuint);
}
extern "C" {
    pub fn alSourcePause(sid: ALuint);
}
extern "C" {
    pub fn alSourceQueueBuffers(sid: ALuint, numEntries: ALsizei, bids: *const ALuint);
}
extern "C" {
    pub fn alSourceUnqueueBuffers(sid: ALuint, numEntries: ALsizei, bids: *mut ALuint);
}
extern "C" {
    #[doc = " BUFFER"]
    #[doc = " Buffer objects are storage space for sample data."]
    #[doc = " Buffers are referred to by Sources. One Buffer can be used"]
    #[doc = " by multiple Sources."]
    #[doc = ""]
    #[doc = " Properties include: -"]
    #[doc = ""]
    #[doc = " Frequency (Query only)    AL_FREQUENCY      ALint"]
    #[doc = " Size (Query only)         AL_SIZE           ALint"]
    #[doc = " Bits (Query only)         AL_BITS           ALint"]
    #[doc = " Channels (Query only)     AL_CHANNELS       ALint"]
    pub fn alGenBuffers(n: ALsizei, buffers: *mut ALuint);
}
extern "C" {
    pub fn alDeleteBuffers(n: ALsizei, buffers: *const ALuint);
}
extern "C" {
    pub fn alIsBuffer(bid: ALuint) -> ALboolean;
}
extern "C" {
    pub fn alBufferData(bid: ALuint, format: ALenum, data: *const ALvoid, size: ALsizei, freq: ALsizei);
}
extern "C" {
    pub fn alBufferf(bid: ALuint, param: ALenum, value: ALfloat);
}
extern "C" {
    pub fn alBuffer3f(bid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
}
extern "C" {
    pub fn alBufferfv(bid: ALuint, param: ALenum, values: *const ALfloat);
}
extern "C" {
    pub fn alBufferi(bid: ALuint, param: ALenum, value: ALint);
}
extern "C" {
    pub fn alBuffer3i(bid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
}
extern "C" {
    pub fn alBufferiv(bid: ALuint, param: ALenum, values: *const ALint);
}
extern "C" {
    pub fn alGetBufferf(bid: ALuint, param: ALenum, value: *mut ALfloat);
}
extern "C" {
    pub fn alGetBuffer3f(bid: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
}
extern "C" {
    pub fn alGetBufferfv(bid: ALuint, param: ALenum, values: *mut ALfloat);
}
extern "C" {
    pub fn alGetBufferi(bid: ALuint, param: ALenum, value: *mut ALint);
}
extern "C" {
    pub fn alGetBuffer3i(bid: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
}
extern "C" {
    pub fn alGetBufferiv(bid: ALuint, param: ALenum, values: *mut ALint);
}
extern "C" {
    pub fn alDopplerFactor(value: ALfloat);
}
extern "C" {
    pub fn alDopplerVelocity(value: ALfloat);
}
extern "C" {
    pub fn alSpeedOfSound(value: ALfloat);
}
extern "C" {
    pub fn alDistanceModel(distanceModel: ALenum);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ALCdevice_struct {
    _unused: [u8; 0],
}
pub type ALCdevice = ALCdevice_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ALCcontext_struct {
    _unused: [u8; 0],
}
pub type ALCcontext = ALCcontext_struct;
#[doc = " 8-bit boolean"]
pub type ALCboolean = ::std::os::raw::c_char;
#[doc = " character"]
pub type ALCchar = ::std::os::raw::c_char;
#[doc = " signed 8-bit 2's complement integer"]
pub type ALCbyte = ::std::os::raw::c_char;
#[doc = " unsigned 8-bit integer"]
pub type ALCubyte = ::std::os::raw::c_uchar;
#[doc = " signed 16-bit 2's complement integer"]
pub type ALCshort = ::std::os::raw::c_short;
#[doc = " unsigned 16-bit integer"]
pub type ALCushort = ::std::os::raw::c_ushort;
#[doc = " signed 32-bit 2's complement integer"]
pub type ALCint = ::std::os::raw::c_int;
#[doc = " unsigned 32-bit integer"]
pub type ALCuint = ::std::os::raw::c_uint;
#[doc = " non-negative 32-bit binary integer size"]
pub type ALCsizei = ::std::os::raw::c_int;
#[doc = " enumerated 32-bit value"]
pub type ALCenum = ::std::os::raw::c_int;
#[doc = " 32-bit IEEE754 floating-point"]
pub type ALCfloat = f32;
#[doc = " 64-bit IEEE754 floating-point"]
pub type ALCdouble = f64;
#[doc = " void type (for opaque pointers only)"]
pub type ALCvoid = ::std::os::raw::c_void;
extern "C" {
    pub fn alcCreateContext(device: *mut ALCdevice, attrlist: *const ALCint) -> *mut ALCcontext;
}
extern "C" {
    pub fn alcMakeContextCurrent(context: *mut ALCcontext) -> ALCboolean;
}
extern "C" {
    pub fn alcProcessContext(context: *mut ALCcontext);
}
extern "C" {
    pub fn alcSuspendContext(context: *mut ALCcontext);
}
extern "C" {
    pub fn alcDestroyContext(context: *mut ALCcontext);
}
extern "C" {
    pub fn alcGetCurrentContext() -> *mut ALCcontext;
}
extern "C" {
    pub fn alcGetContextsDevice(context: *mut ALCcontext) -> *mut ALCdevice;
}
extern "C" {
    pub fn alcOpenDevice(devicename: *const ALCchar) -> *mut ALCdevice;
}
extern "C" {
    pub fn alcCloseDevice(device: *mut ALCdevice) -> ALCboolean;
}
extern "C" {
    pub fn alcGetError(device: *mut ALCdevice) -> ALCenum;
}
extern "C" {
    pub fn alcIsExtensionPresent(device: *mut ALCdevice, extname: *const ALCchar) -> ALCboolean;
}
extern "C" {
    pub fn alcGetProcAddress(device: *mut ALCdevice, funcname: *const ALCchar) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn alcGetEnumValue(device: *mut ALCdevice, enumname: *const ALCchar) -> ALCenum;
}
extern "C" {
    pub fn alcGetString(device: *mut ALCdevice, param: ALCenum) -> *const ALCchar;
}
extern "C" {
    pub fn alcGetIntegerv(device: *mut ALCdevice, param: ALCenum, size: ALCsizei, data: *mut ALCint);
}
extern "C" {
    pub fn alcCaptureOpenDevice(devicename: *const ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *mut ALCdevice;
}
extern "C" {
    pub fn alcCaptureCloseDevice(device: *mut ALCdevice) -> ALCboolean;
}
extern "C" {
    pub fn alcCaptureStart(device: *mut ALCdevice);
}
extern "C" {
    pub fn alcCaptureStop(device: *mut ALCdevice);
}
extern "C" {
    pub fn alcCaptureSamples(device: *mut ALCdevice, buffer: *mut ALCvoid, samples: ALCsizei);
}
pub type LPALCCREATECONTEXT = unsafe extern "C" fn(device: *mut ALCdevice, attrlist: *const ALCint) -> *mut ALCcontext;
pub type LPALCMAKECONTEXTCURRENT = unsafe extern "C" fn(context: *mut ALCcontext) -> ALCboolean;
pub type LPALCPROCESSCONTEXT = unsafe extern "C" fn(context: *mut ALCcontext);
pub type LPALCSUSPENDCONTEXT = unsafe extern "C" fn(context: *mut ALCcontext);
pub type LPALCDESTROYCONTEXT = unsafe extern "C" fn(context: *mut ALCcontext);
pub type LPALCGETCURRENTCONTEXT = unsafe extern "C" fn() -> *mut ALCcontext;
pub type LPALCGETCONTEXTSDEVICE = unsafe extern "C" fn(context: *mut ALCcontext) -> *mut ALCdevice;
pub type LPALCOPENDEVICE = unsafe extern "C" fn(devicename: *const ALCchar) -> *mut ALCdevice;
pub type LPALCCLOSEDEVICE = unsafe extern "C" fn(device: *mut ALCdevice) -> ALCboolean;
pub type LPALCGETERROR = unsafe extern "C" fn(device: *mut ALCdevice) -> ALCenum;
pub type LPALCISEXTENSIONPRESENT = unsafe extern "C" fn(device: *mut ALCdevice, extname: *const ALCchar) -> ALCboolean;
pub type LPALCGETPROCADDRESS = unsafe extern "C" fn(device: *mut ALCdevice, funcname: *const ALCchar) -> *mut ::std::os::raw::c_void;
pub type LPALCGETENUMVALUE = unsafe extern "C" fn(device: *mut ALCdevice, enumname: *const ALCchar) -> ALCenum;
pub type LPALCGETSTRING = unsafe extern "C" fn(device: *mut ALCdevice, param: ALCenum) -> *const ALCchar;
pub type LPALCGETINTEGERV = unsafe extern "C" fn(device: *mut ALCdevice, param: ALCenum, size: ALCsizei, dest: *mut ALCint);
pub type LPALCCAPTUREOPENDEVICE = unsafe extern "C" fn(devicename: *const ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *mut ALCdevice;
pub type LPALCCAPTURECLOSEDEVICE = unsafe extern "C" fn(device: *mut ALCdevice) -> ALCboolean;
pub type LPALCCAPTURESTART = unsafe extern "C" fn(device: *mut ALCdevice);
pub type LPALCCAPTURESTOP = unsafe extern "C" fn(device: *mut ALCdevice);
pub type LPALCCAPTURESAMPLES = unsafe extern "C" fn(device: *mut ALCdevice, buffer: *mut ALCvoid, samples: ALCsizei);