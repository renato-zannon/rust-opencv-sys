/* automatically generated by rust-bindgen */

use core::{CvArr, CvFont, CvMat, CvPoint, CvScalar, CvSize, IplImage};

pub type Enum_Unnamed1 = ::libc::c_uint;
pub const CV_FONT_LIGHT: ::libc::c_uint = 25;
pub const CV_FONT_NORMAL: ::libc::c_uint = 50;
pub const CV_FONT_DEMIBOLD: ::libc::c_uint = 63;
pub const CV_FONT_BOLD: ::libc::c_uint = 75;
pub const CV_FONT_BLACK: ::libc::c_uint = 87;
pub type Enum_Unnamed2 = ::libc::c_uint;
pub const CV_STYLE_NORMAL: ::libc::c_uint = 0;
pub const CV_STYLE_ITALIC: ::libc::c_uint = 1;
pub const CV_STYLE_OBLIQUE: ::libc::c_uint = 2;
pub type CvButtonCallback =
    ::std::option::Option<extern "C" fn
                              (state: ::libc::c_int,
                               userdata: *mut ::libc::c_void) -> ()>;
pub type Enum_Unnamed3 = ::libc::c_uint;
pub const CV_PUSH_BUTTON: ::libc::c_uint = 0;
pub const CV_CHECKBOX: ::libc::c_uint = 1;
pub const CV_RADIOBOX: ::libc::c_uint = 2;
pub type Enum_Unnamed4 = ::libc::c_uint;
pub const CV_WND_PROP_FULLSCREEN: ::libc::c_uint = 0;
pub const CV_WND_PROP_AUTOSIZE: ::libc::c_uint = 1;
pub const CV_WND_PROP_ASPECTRATIO: ::libc::c_uint = 2;
pub const CV_WND_PROP_OPENGL: ::libc::c_uint = 3;
pub const CV_WINDOW_NORMAL: ::libc::c_uint = 0;
pub const CV_WINDOW_AUTOSIZE: ::libc::c_uint = 1;
pub const CV_WINDOW_OPENGL: ::libc::c_uint = 4096;
pub const CV_GUI_EXPANDED: ::libc::c_uint = 0;
pub const CV_GUI_NORMAL: ::libc::c_uint = 16;
pub const CV_WINDOW_FULLSCREEN: ::libc::c_uint = 1;
pub const CV_WINDOW_FREERATIO: ::libc::c_uint = 256;
pub const CV_WINDOW_KEEPRATIO: ::libc::c_uint = 0;
pub type CvTrackbarCallback =
    ::std::option::Option<extern "C" fn(pos: ::libc::c_int) -> ()>;
pub type CvTrackbarCallback2 =
    ::std::option::Option<extern "C" fn
                              (pos: ::libc::c_int,
                               userdata: *mut ::libc::c_void) -> ()>;
pub type Enum_Unnamed5 = ::libc::c_uint;
pub const CV_EVENT_MOUSEMOVE: ::libc::c_uint = 0;
pub const CV_EVENT_LBUTTONDOWN: ::libc::c_uint = 1;
pub const CV_EVENT_RBUTTONDOWN: ::libc::c_uint = 2;
pub const CV_EVENT_MBUTTONDOWN: ::libc::c_uint = 3;
pub const CV_EVENT_LBUTTONUP: ::libc::c_uint = 4;
pub const CV_EVENT_RBUTTONUP: ::libc::c_uint = 5;
pub const CV_EVENT_MBUTTONUP: ::libc::c_uint = 6;
pub const CV_EVENT_LBUTTONDBLCLK: ::libc::c_uint = 7;
pub const CV_EVENT_RBUTTONDBLCLK: ::libc::c_uint = 8;
pub const CV_EVENT_MBUTTONDBLCLK: ::libc::c_uint = 9;
pub type Enum_Unnamed6 = ::libc::c_uint;
pub const CV_EVENT_FLAG_LBUTTON: ::libc::c_uint = 1;
pub const CV_EVENT_FLAG_RBUTTON: ::libc::c_uint = 2;
pub const CV_EVENT_FLAG_MBUTTON: ::libc::c_uint = 4;
pub const CV_EVENT_FLAG_CTRLKEY: ::libc::c_uint = 8;
pub const CV_EVENT_FLAG_SHIFTKEY: ::libc::c_uint = 16;
pub const CV_EVENT_FLAG_ALTKEY: ::libc::c_uint = 32;
pub type CvMouseCallback =
    ::std::option::Option<extern "C" fn
                              (event: ::libc::c_int, x: ::libc::c_int,
                               y: ::libc::c_int, flags: ::libc::c_int,
                               param: *mut ::libc::c_void) -> ()>;
pub type Enum_Unnamed7 = ::libc::c_int;
pub const CV_LOAD_IMAGE_UNCHANGED: ::libc::c_int = -1;
pub const CV_LOAD_IMAGE_GRAYSCALE: ::libc::c_int = 0;
pub const CV_LOAD_IMAGE_COLOR: ::libc::c_int = 1;
pub const CV_LOAD_IMAGE_ANYDEPTH: ::libc::c_int = 2;
pub const CV_LOAD_IMAGE_ANYCOLOR: ::libc::c_int = 4;
pub type Enum_Unnamed8 = ::libc::c_uint;
pub const CV_IMWRITE_JPEG_QUALITY: ::libc::c_uint = 1;
pub const CV_IMWRITE_PNG_COMPRESSION: ::libc::c_uint = 16;
pub const CV_IMWRITE_PNG_STRATEGY: ::libc::c_uint = 17;
pub const CV_IMWRITE_PNG_BILEVEL: ::libc::c_uint = 18;
pub const CV_IMWRITE_PNG_STRATEGY_DEFAULT: ::libc::c_uint = 0;
pub const CV_IMWRITE_PNG_STRATEGY_FILTERED: ::libc::c_uint = 1;
pub const CV_IMWRITE_PNG_STRATEGY_HUFFMAN_ONLY: ::libc::c_uint = 2;
pub const CV_IMWRITE_PNG_STRATEGY_RLE: ::libc::c_uint = 3;
pub const CV_IMWRITE_PNG_STRATEGY_FIXED: ::libc::c_uint = 4;
pub const CV_IMWRITE_PXM_BINARY: ::libc::c_uint = 32;
pub type Enum_Unnamed9 = ::libc::c_uint;
pub const CV_CVTIMG_FLIP: ::libc::c_uint = 1;
pub const CV_CVTIMG_SWAP_RB: ::libc::c_uint = 2;
pub type CvOpenGlDrawCallback =
    ::std::option::Option<extern "C" fn(userdata: *mut ::libc::c_void) -> ()>;
pub enum Struct_CvCapture { }
pub type CvCapture = Struct_CvCapture;
pub type Enum_Unnamed10 = ::libc::c_uint;
pub const CV_CAP_ANY: ::libc::c_uint = 0;
pub const CV_CAP_MIL: ::libc::c_uint = 100;
pub const CV_CAP_VFW: ::libc::c_uint = 200;
pub const CV_CAP_V4L: ::libc::c_uint = 200;
pub const CV_CAP_V4L2: ::libc::c_uint = 200;
pub const CV_CAP_FIREWARE: ::libc::c_uint = 300;
pub const CV_CAP_FIREWIRE: ::libc::c_uint = 300;
pub const CV_CAP_IEEE1394: ::libc::c_uint = 300;
pub const CV_CAP_DC1394: ::libc::c_uint = 300;
pub const CV_CAP_CMU1394: ::libc::c_uint = 300;
pub const CV_CAP_STEREO: ::libc::c_uint = 400;
pub const CV_CAP_TYZX: ::libc::c_uint = 400;
pub const CV_TYZX_LEFT: ::libc::c_uint = 400;
pub const CV_TYZX_RIGHT: ::libc::c_uint = 401;
pub const CV_TYZX_COLOR: ::libc::c_uint = 402;
pub const CV_TYZX_Z: ::libc::c_uint = 403;
pub const CV_CAP_QT: ::libc::c_uint = 500;
pub const CV_CAP_UNICAP: ::libc::c_uint = 600;
pub const CV_CAP_DSHOW: ::libc::c_uint = 700;
pub const CV_CAP_MSMF: ::libc::c_uint = 1400;
pub const CV_CAP_PVAPI: ::libc::c_uint = 800;
pub const CV_CAP_OPENNI: ::libc::c_uint = 900;
pub const CV_CAP_OPENNI_ASUS: ::libc::c_uint = 910;
pub const CV_CAP_ANDROID: ::libc::c_uint = 1000;
pub const CV_CAP_ANDROID_BACK: ::libc::c_uint = 1099;
pub const CV_CAP_ANDROID_FRONT: ::libc::c_uint = 1098;
pub const CV_CAP_XIAPI: ::libc::c_uint = 1100;
pub const CV_CAP_AVFOUNDATION: ::libc::c_uint = 1200;
pub const CV_CAP_GIGANETIX: ::libc::c_uint = 1300;
pub const CV_CAP_INTELPERC: ::libc::c_uint = 1500;
pub type Enum_Unnamed11 = ::libc::c_int;
pub const CV_CAP_PROP_DC1394_OFF: ::libc::c_int = -4;
pub const CV_CAP_PROP_DC1394_MODE_MANUAL: ::libc::c_int = -3;
pub const CV_CAP_PROP_DC1394_MODE_AUTO: ::libc::c_int = -2;
pub const CV_CAP_PROP_DC1394_MODE_ONE_PUSH_AUTO: ::libc::c_int = -1;
pub const CV_CAP_PROP_POS_MSEC: ::libc::c_int = 0;
pub const CV_CAP_PROP_POS_FRAMES: ::libc::c_int = 1;
pub const CV_CAP_PROP_POS_AVI_RATIO: ::libc::c_int = 2;
pub const CV_CAP_PROP_FRAME_WIDTH: ::libc::c_int = 3;
pub const CV_CAP_PROP_FRAME_HEIGHT: ::libc::c_int = 4;
pub const CV_CAP_PROP_FPS: ::libc::c_int = 5;
pub const CV_CAP_PROP_FOURCC: ::libc::c_int = 6;
pub const CV_CAP_PROP_FRAME_COUNT: ::libc::c_int = 7;
pub const CV_CAP_PROP_FORMAT: ::libc::c_int = 8;
pub const CV_CAP_PROP_MODE: ::libc::c_int = 9;
pub const CV_CAP_PROP_BRIGHTNESS: ::libc::c_int = 10;
pub const CV_CAP_PROP_CONTRAST: ::libc::c_int = 11;
pub const CV_CAP_PROP_SATURATION: ::libc::c_int = 12;
pub const CV_CAP_PROP_HUE: ::libc::c_int = 13;
pub const CV_CAP_PROP_GAIN: ::libc::c_int = 14;
pub const CV_CAP_PROP_EXPOSURE: ::libc::c_int = 15;
pub const CV_CAP_PROP_CONVERT_RGB: ::libc::c_int = 16;
pub const CV_CAP_PROP_WHITE_BALANCE_BLUE_U: ::libc::c_int = 17;
pub const CV_CAP_PROP_RECTIFICATION: ::libc::c_int = 18;
pub const CV_CAP_PROP_MONOCROME: ::libc::c_int = 19;
pub const CV_CAP_PROP_SHARPNESS: ::libc::c_int = 20;
pub const CV_CAP_PROP_AUTO_EXPOSURE: ::libc::c_int = 21;
pub const CV_CAP_PROP_GAMMA: ::libc::c_int = 22;
pub const CV_CAP_PROP_TEMPERATURE: ::libc::c_int = 23;
pub const CV_CAP_PROP_TRIGGER: ::libc::c_int = 24;
pub const CV_CAP_PROP_TRIGGER_DELAY: ::libc::c_int = 25;
pub const CV_CAP_PROP_WHITE_BALANCE_RED_V: ::libc::c_int = 26;
pub const CV_CAP_PROP_ZOOM: ::libc::c_int = 27;
pub const CV_CAP_PROP_FOCUS: ::libc::c_int = 28;
pub const CV_CAP_PROP_GUID: ::libc::c_int = 29;
pub const CV_CAP_PROP_ISO_SPEED: ::libc::c_int = 30;
pub const CV_CAP_PROP_MAX_DC1394: ::libc::c_int = 31;
pub const CV_CAP_PROP_BACKLIGHT: ::libc::c_int = 32;
pub const CV_CAP_PROP_PAN: ::libc::c_int = 33;
pub const CV_CAP_PROP_TILT: ::libc::c_int = 34;
pub const CV_CAP_PROP_ROLL: ::libc::c_int = 35;
pub const CV_CAP_PROP_IRIS: ::libc::c_int = 36;
pub const CV_CAP_PROP_SETTINGS: ::libc::c_int = 37;
pub const CV_CAP_PROP_AUTOGRAB: ::libc::c_int = 1024;
pub const CV_CAP_PROP_SUPPORTED_PREVIEW_SIZES_STRING: ::libc::c_int = 1025;
pub const CV_CAP_PROP_PREVIEW_FORMAT: ::libc::c_int = 1026;
pub const CV_CAP_OPENNI_DEPTH_GENERATOR: ::libc::c_int = -2147483648;
pub const CV_CAP_OPENNI_IMAGE_GENERATOR: ::libc::c_int = 1073741824;
pub const CV_CAP_OPENNI_GENERATORS_MASK: ::libc::c_int = -1073741824;
pub const CV_CAP_PROP_OPENNI_OUTPUT_MODE: ::libc::c_int = 100;
pub const CV_CAP_PROP_OPENNI_FRAME_MAX_DEPTH: ::libc::c_int = 101;
pub const CV_CAP_PROP_OPENNI_BASELINE: ::libc::c_int = 102;
pub const CV_CAP_PROP_OPENNI_FOCAL_LENGTH: ::libc::c_int = 103;
pub const CV_CAP_PROP_OPENNI_REGISTRATION: ::libc::c_int = 104;
pub const CV_CAP_PROP_OPENNI_REGISTRATION_ON: ::libc::c_int = 104;
pub const CV_CAP_PROP_OPENNI_APPROX_FRAME_SYNC: ::libc::c_int = 105;
pub const CV_CAP_PROP_OPENNI_MAX_BUFFER_SIZE: ::libc::c_int = 106;
pub const CV_CAP_PROP_OPENNI_CIRCLE_BUFFER: ::libc::c_int = 107;
pub const CV_CAP_PROP_OPENNI_MAX_TIME_DURATION: ::libc::c_int = 108;
pub const CV_CAP_PROP_OPENNI_GENERATOR_PRESENT: ::libc::c_int = 109;
pub const CV_CAP_OPENNI_IMAGE_GENERATOR_PRESENT: ::libc::c_int = 1073741933;
pub const CV_CAP_OPENNI_IMAGE_GENERATOR_OUTPUT_MODE: ::libc::c_int =
    1073741924;
pub const CV_CAP_OPENNI_DEPTH_GENERATOR_BASELINE: ::libc::c_int = -2147483546;
pub const CV_CAP_OPENNI_DEPTH_GENERATOR_FOCAL_LENGTH: ::libc::c_int =
    -2147483545;
pub const CV_CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION: ::libc::c_int =
    -2147483544;
pub const CV_CAP_OPENNI_DEPTH_GENERATOR_REGISTRATION_ON: ::libc::c_int =
    -2147483544;
pub const CV_CAP_GSTREAMER_QUEUE_LENGTH: ::libc::c_int = 200;
pub const CV_CAP_PROP_PVAPI_MULTICASTIP: ::libc::c_int = 300;
pub const CV_CAP_PROP_XI_DOWNSAMPLING: ::libc::c_int = 400;
pub const CV_CAP_PROP_XI_DATA_FORMAT: ::libc::c_int = 401;
pub const CV_CAP_PROP_XI_OFFSET_X: ::libc::c_int = 402;
pub const CV_CAP_PROP_XI_OFFSET_Y: ::libc::c_int = 403;
pub const CV_CAP_PROP_XI_TRG_SOURCE: ::libc::c_int = 404;
pub const CV_CAP_PROP_XI_TRG_SOFTWARE: ::libc::c_int = 405;
pub const CV_CAP_PROP_XI_GPI_SELECTOR: ::libc::c_int = 406;
pub const CV_CAP_PROP_XI_GPI_MODE: ::libc::c_int = 407;
pub const CV_CAP_PROP_XI_GPI_LEVEL: ::libc::c_int = 408;
pub const CV_CAP_PROP_XI_GPO_SELECTOR: ::libc::c_int = 409;
pub const CV_CAP_PROP_XI_GPO_MODE: ::libc::c_int = 410;
pub const CV_CAP_PROP_XI_LED_SELECTOR: ::libc::c_int = 411;
pub const CV_CAP_PROP_XI_LED_MODE: ::libc::c_int = 412;
pub const CV_CAP_PROP_XI_MANUAL_WB: ::libc::c_int = 413;
pub const CV_CAP_PROP_XI_AUTO_WB: ::libc::c_int = 414;
pub const CV_CAP_PROP_XI_AEAG: ::libc::c_int = 415;
pub const CV_CAP_PROP_XI_EXP_PRIORITY: ::libc::c_int = 416;
pub const CV_CAP_PROP_XI_AE_MAX_LIMIT: ::libc::c_int = 417;
pub const CV_CAP_PROP_XI_AG_MAX_LIMIT: ::libc::c_int = 418;
pub const CV_CAP_PROP_XI_AEAG_LEVEL: ::libc::c_int = 419;
pub const CV_CAP_PROP_XI_TIMEOUT: ::libc::c_int = 420;
pub const CV_CAP_PROP_ANDROID_FLASH_MODE: ::libc::c_int = 8001;
pub const CV_CAP_PROP_ANDROID_FOCUS_MODE: ::libc::c_int = 8002;
pub const CV_CAP_PROP_ANDROID_WHITE_BALANCE: ::libc::c_int = 8003;
pub const CV_CAP_PROP_ANDROID_ANTIBANDING: ::libc::c_int = 8004;
pub const CV_CAP_PROP_ANDROID_FOCAL_LENGTH: ::libc::c_int = 8005;
pub const CV_CAP_PROP_ANDROID_FOCUS_DISTANCE_NEAR: ::libc::c_int = 8006;
pub const CV_CAP_PROP_ANDROID_FOCUS_DISTANCE_OPTIMAL: ::libc::c_int = 8007;
pub const CV_CAP_PROP_ANDROID_FOCUS_DISTANCE_FAR: ::libc::c_int = 8008;
pub const CV_CAP_PROP_ANDROID_EXPOSE_LOCK: ::libc::c_int = 8009;
pub const CV_CAP_PROP_ANDROID_WHITEBALANCE_LOCK: ::libc::c_int = 8010;
pub const CV_CAP_PROP_IOS_DEVICE_FOCUS: ::libc::c_int = 9001;
pub const CV_CAP_PROP_IOS_DEVICE_EXPOSURE: ::libc::c_int = 9002;
pub const CV_CAP_PROP_IOS_DEVICE_FLASH: ::libc::c_int = 9003;
pub const CV_CAP_PROP_IOS_DEVICE_WHITEBALANCE: ::libc::c_int = 9004;
pub const CV_CAP_PROP_IOS_DEVICE_TORCH: ::libc::c_int = 9005;
pub const CV_CAP_PROP_GIGA_FRAME_OFFSET_X: ::libc::c_int = 10001;
pub const CV_CAP_PROP_GIGA_FRAME_OFFSET_Y: ::libc::c_int = 10002;
pub const CV_CAP_PROP_GIGA_FRAME_WIDTH_MAX: ::libc::c_int = 10003;
pub const CV_CAP_PROP_GIGA_FRAME_HEIGH_MAX: ::libc::c_int = 10004;
pub const CV_CAP_PROP_GIGA_FRAME_SENS_WIDTH: ::libc::c_int = 10005;
pub const CV_CAP_PROP_GIGA_FRAME_SENS_HEIGH: ::libc::c_int = 10006;
pub const CV_CAP_PROP_INTELPERC_PROFILE_COUNT: ::libc::c_int = 11001;
pub const CV_CAP_PROP_INTELPERC_PROFILE_IDX: ::libc::c_int = 11002;
pub const CV_CAP_PROP_INTELPERC_DEPTH_LOW_CONFIDENCE_VALUE: ::libc::c_int =
    11003;
pub const CV_CAP_PROP_INTELPERC_DEPTH_SATURATION_VALUE: ::libc::c_int = 11004;
pub const CV_CAP_PROP_INTELPERC_DEPTH_CONFIDENCE_THRESHOLD: ::libc::c_int =
    11005;
pub const CV_CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_HORZ: ::libc::c_int =
    11006;
pub const CV_CAP_PROP_INTELPERC_DEPTH_FOCAL_LENGTH_VERT: ::libc::c_int =
    11007;
pub const CV_CAP_INTELPERC_DEPTH_GENERATOR: ::libc::c_int = 536870912;
pub const CV_CAP_INTELPERC_IMAGE_GENERATOR: ::libc::c_int = 268435456;
pub const CV_CAP_INTELPERC_GENERATORS_MASK: ::libc::c_int = 805306368;
pub type Enum_Unnamed12 = ::libc::c_uint;
pub const CV_CAP_OPENNI_DEPTH_MAP: ::libc::c_uint = 0;
pub const CV_CAP_OPENNI_POINT_CLOUD_MAP: ::libc::c_uint = 1;
pub const CV_CAP_OPENNI_DISPARITY_MAP: ::libc::c_uint = 2;
pub const CV_CAP_OPENNI_DISPARITY_MAP_32F: ::libc::c_uint = 3;
pub const CV_CAP_OPENNI_VALID_DEPTH_MASK: ::libc::c_uint = 4;
pub const CV_CAP_OPENNI_BGR_IMAGE: ::libc::c_uint = 5;
pub const CV_CAP_OPENNI_GRAY_IMAGE: ::libc::c_uint = 6;
pub type Enum_Unnamed13 = ::libc::c_uint;
pub const CV_CAP_OPENNI_VGA_30HZ: ::libc::c_uint = 0;
pub const CV_CAP_OPENNI_SXGA_15HZ: ::libc::c_uint = 1;
pub const CV_CAP_OPENNI_SXGA_30HZ: ::libc::c_uint = 2;
pub const CV_CAP_OPENNI_QVGA_30HZ: ::libc::c_uint = 3;
pub const CV_CAP_OPENNI_QVGA_60HZ: ::libc::c_uint = 4;
pub type Enum_Unnamed14 = ::libc::c_uint;
pub const CV_CAP_ANDROID_COLOR_FRAME_BGR: ::libc::c_uint = 0;
pub const CV_CAP_ANDROID_COLOR_FRAME: ::libc::c_uint = 0;
pub const CV_CAP_ANDROID_GREY_FRAME: ::libc::c_uint = 1;
pub const CV_CAP_ANDROID_COLOR_FRAME_RGB: ::libc::c_uint = 2;
pub const CV_CAP_ANDROID_COLOR_FRAME_BGRA: ::libc::c_uint = 3;
pub const CV_CAP_ANDROID_COLOR_FRAME_RGBA: ::libc::c_uint = 4;
pub type Enum_Unnamed15 = ::libc::c_uint;
pub const CV_CAP_ANDROID_FLASH_MODE_AUTO: ::libc::c_uint = 0;
pub const CV_CAP_ANDROID_FLASH_MODE_OFF: ::libc::c_uint = 1;
pub const CV_CAP_ANDROID_FLASH_MODE_ON: ::libc::c_uint = 2;
pub const CV_CAP_ANDROID_FLASH_MODE_RED_EYE: ::libc::c_uint = 3;
pub const CV_CAP_ANDROID_FLASH_MODE_TORCH: ::libc::c_uint = 4;
pub type Enum_Unnamed16 = ::libc::c_uint;
pub const CV_CAP_ANDROID_FOCUS_MODE_AUTO: ::libc::c_uint = 0;
pub const CV_CAP_ANDROID_FOCUS_MODE_CONTINUOUS_PICTURE: ::libc::c_uint = 1;
pub const CV_CAP_ANDROID_FOCUS_MODE_CONTINUOUS_VIDEO: ::libc::c_uint = 2;
pub const CV_CAP_ANDROID_FOCUS_MODE_EDOF: ::libc::c_uint = 3;
pub const CV_CAP_ANDROID_FOCUS_MODE_FIXED: ::libc::c_uint = 4;
pub const CV_CAP_ANDROID_FOCUS_MODE_INFINITY: ::libc::c_uint = 5;
pub const CV_CAP_ANDROID_FOCUS_MODE_MACRO: ::libc::c_uint = 6;
pub type Enum_Unnamed17 = ::libc::c_uint;
pub const CV_CAP_ANDROID_WHITE_BALANCE_AUTO: ::libc::c_uint = 0;
pub const CV_CAP_ANDROID_WHITE_BALANCE_CLOUDY_DAYLIGHT: ::libc::c_uint = 1;
pub const CV_CAP_ANDROID_WHITE_BALANCE_DAYLIGHT: ::libc::c_uint = 2;
pub const CV_CAP_ANDROID_WHITE_BALANCE_FLUORESCENT: ::libc::c_uint = 3;
pub const CV_CAP_ANDROID_WHITE_BALANCE_INCANDESCENT: ::libc::c_uint = 4;
pub const CV_CAP_ANDROID_WHITE_BALANCE_SHADE: ::libc::c_uint = 5;
pub const CV_CAP_ANDROID_WHITE_BALANCE_TWILIGHT: ::libc::c_uint = 6;
pub const CV_CAP_ANDROID_WHITE_BALANCE_WARM_FLUORESCENT: ::libc::c_uint = 7;
pub type Enum_Unnamed18 = ::libc::c_uint;
pub const CV_CAP_ANDROID_ANTIBANDING_50HZ: ::libc::c_uint = 0;
pub const CV_CAP_ANDROID_ANTIBANDING_60HZ: ::libc::c_uint = 1;
pub const CV_CAP_ANDROID_ANTIBANDING_AUTO: ::libc::c_uint = 2;
pub const CV_CAP_ANDROID_ANTIBANDING_OFF: ::libc::c_uint = 3;
pub type Enum_Unnamed19 = ::libc::c_uint;
pub const CV_CAP_INTELPERC_DEPTH_MAP: ::libc::c_uint = 0;
pub const CV_CAP_INTELPERC_UVDEPTH_MAP: ::libc::c_uint = 1;
pub const CV_CAP_INTELPERC_IR_MAP: ::libc::c_uint = 2;
pub const CV_CAP_INTELPERC_IMAGE: ::libc::c_uint = 3;
pub enum Struct_CvVideoWriter { }
pub type CvVideoWriter = Struct_CvVideoWriter;

extern "C" {
    pub fn cvFontQt(nameFont: *const ::libc::c_char, pointSize: ::libc::c_int,
                    color: CvScalar, weight: ::libc::c_int,
                    style: ::libc::c_int, spacing: ::libc::c_int) -> CvFont;
    pub fn cvAddText(img: *const CvArr, text: *const ::libc::c_char,
                     org: CvPoint, arg2: *mut CvFont) -> ();
    pub fn cvDisplayOverlay(name: *const ::libc::c_char,
                            text: *const ::libc::c_char,
                            delayms: ::libc::c_int) -> ();
    pub fn cvDisplayStatusBar(name: *const ::libc::c_char,
                              text: *const ::libc::c_char,
                              delayms: ::libc::c_int) -> ();
    pub fn cvSaveWindowParameters(name: *const ::libc::c_char) -> ();
    pub fn cvLoadWindowParameters(name: *const ::libc::c_char) -> ();
    pub fn cvStartLoop(pt2Func:
                           ::std::option::Option<extern "C" fn
                                                     (argc: ::libc::c_int,
                                                      argv:
                                                          *mut *mut ::libc::c_char)
                                                     -> ::libc::c_int>,
                       argc: ::libc::c_int, argv: *mut *mut ::libc::c_char)
     -> ::libc::c_int;
    pub fn cvStopLoop() -> ();
    pub fn cvCreateButton(button_name: *const ::libc::c_char,
                          on_change: CvButtonCallback,
                          userdata: *mut ::libc::c_void,
                          button_type: ::libc::c_int,
                          initial_button_state: ::libc::c_int)
     -> ::libc::c_int;
    pub fn cvInitSystem(argc: ::libc::c_int, argv: *mut *mut ::libc::c_char)
     -> ::libc::c_int;
    pub fn cvStartWindowThread() -> ::libc::c_int;
    pub fn cvNamedWindow(name: *const ::libc::c_char, flags: ::libc::c_int)
     -> ::libc::c_int;
    pub fn cvSetWindowProperty(name: *const ::libc::c_char,
                               prop_id: ::libc::c_int,
                               prop_value: ::libc::c_double) -> ();
    pub fn cvGetWindowProperty(name: *const ::libc::c_char,
                               prop_id: ::libc::c_int) -> ::libc::c_double;
    pub fn cvShowImage(name: *const ::libc::c_char, image: *const CvArr)
     -> ();
    pub fn cvResizeWindow(name: *const ::libc::c_char, width: ::libc::c_int,
                          height: ::libc::c_int) -> ();
    pub fn cvMoveWindow(name: *const ::libc::c_char, x: ::libc::c_int,
                        y: ::libc::c_int) -> ();
    pub fn cvDestroyWindow(name: *const ::libc::c_char) -> ();
    pub fn cvDestroyAllWindows() -> ();
    pub fn cvGetWindowHandle(name: *const ::libc::c_char)
     -> *mut ::libc::c_void;
    pub fn cvGetWindowName(window_handle: *mut ::libc::c_void)
     -> *const ::libc::c_char;
    pub fn cvCreateTrackbar(trackbar_name: *const ::libc::c_char,
                            window_name: *const ::libc::c_char,
                            value: *mut ::libc::c_int, count: ::libc::c_int,
                            on_change: CvTrackbarCallback) -> ::libc::c_int;
    pub fn cvCreateTrackbar2(trackbar_name: *const ::libc::c_char,
                             window_name: *const ::libc::c_char,
                             value: *mut ::libc::c_int, count: ::libc::c_int,
                             on_change: CvTrackbarCallback2,
                             userdata: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn cvGetTrackbarPos(trackbar_name: *const ::libc::c_char,
                            window_name: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn cvSetTrackbarPos(trackbar_name: *const ::libc::c_char,
                            window_name: *const ::libc::c_char,
                            pos: ::libc::c_int) -> ();
    pub fn cvSetMouseCallback(window_name: *const ::libc::c_char,
                              on_mouse: CvMouseCallback,
                              param: *mut ::libc::c_void) -> ();
    pub fn cvLoadImage(filename: *const ::libc::c_char,
                       iscolor: ::libc::c_int) -> *mut IplImage;
    pub fn cvLoadImageM(filename: *const ::libc::c_char,
                        iscolor: ::libc::c_int) -> *mut CvMat;
    pub fn cvSaveImage(filename: *const ::libc::c_char, image: *const CvArr,
                       params: *const ::libc::c_int) -> ::libc::c_int;
    pub fn cvDecodeImage(buf: *const CvMat, iscolor: ::libc::c_int)
     -> *mut IplImage;
    pub fn cvDecodeImageM(buf: *const CvMat, iscolor: ::libc::c_int)
     -> *mut CvMat;
    pub fn cvEncodeImage(ext: *const ::libc::c_char, image: *const CvArr,
                         params: *const ::libc::c_int) -> *mut CvMat;
    pub fn cvConvertImage(src: *const CvArr, dst: *mut CvArr,
                          flags: ::libc::c_int) -> ();
    pub fn cvWaitKey(delay: ::libc::c_int) -> ::libc::c_int;
    pub fn cvSetOpenGlDrawCallback(window_name: *const ::libc::c_char,
                                   callback: CvOpenGlDrawCallback,
                                   userdata: *mut ::libc::c_void) -> ();
    pub fn cvSetOpenGlContext(window_name: *const ::libc::c_char) -> ();
    pub fn cvUpdateWindow(window_name: *const ::libc::c_char) -> ();
    pub fn cvCreateFileCapture(filename: *const ::libc::c_char)
     -> *mut CvCapture;
    pub fn cvCreateCameraCapture(index: ::libc::c_int) -> *mut CvCapture;
    pub fn cvGrabFrame(capture: *mut CvCapture) -> ::libc::c_int;
    pub fn cvRetrieveFrame(capture: *mut CvCapture, streamIdx: ::libc::c_int)
     -> *mut IplImage;
    pub fn cvQueryFrame(capture: *mut CvCapture) -> *mut IplImage;
    pub fn cvReleaseCapture(capture: *mut *mut CvCapture) -> ();
    pub fn cvGetCaptureProperty(capture: *mut CvCapture,
                                property_id: ::libc::c_int)
     -> ::libc::c_double;
    pub fn cvSetCaptureProperty(capture: *mut CvCapture,
                                property_id: ::libc::c_int,
                                value: ::libc::c_double) -> ::libc::c_int;
    pub fn cvGetCaptureDomain(capture: *mut CvCapture) -> ::libc::c_int;
    pub fn cvCreateVideoWriter(filename: *const ::libc::c_char,
                               fourcc: ::libc::c_int, fps: ::libc::c_double,
                               frame_size: CvSize, is_color: ::libc::c_int)
     -> *mut CvVideoWriter;
    pub fn cvWriteFrame(writer: *mut CvVideoWriter, image: *const IplImage)
     -> ::libc::c_int;
    pub fn cvReleaseVideoWriter(writer: *mut *mut CvVideoWriter) -> ();
}
