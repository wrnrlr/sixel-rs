pub const SIXEL_OK: ::std::os::raw::c_uint = 0;
pub const SIXEL_FALSE: ::std::os::raw::c_uint = 4096;
pub const SIXEL_INTERRUPTED: ::std::os::raw::c_uint = 1;
pub const SIXEL_BAD_ALLOCATION: ::std::os::raw::c_uint = 4353;
pub const SIXEL_BAD_ARGUMENT: ::std::os::raw::c_uint = 4354;
pub const SIXEL_BAD_INPUT: ::std::os::raw::c_uint = 4355;
pub const SIXEL_RUNTIME_ERROR: ::std::os::raw::c_uint = 4352;
pub const SIXEL_LOGIC_ERROR: ::std::os::raw::c_uint = 4608;
pub const SIXEL_NOT_IMPLEMENTED: ::std::os::raw::c_uint = 4865;
pub const SIXEL_FEATURE_ERROR: ::std::os::raw::c_uint = 4864;
pub const SIXEL_STBI_ERROR: ::std::os::raw::c_uint = 6656;
pub const SIXEL_STBIW_ERROR: ::std::os::raw::c_uint = 6912;
pub const SIXEL_JPEG_ERROR: ::std::os::raw::c_uint = 5632;
pub const SIXEL_PNG_ERROR: ::std::os::raw::c_uint = 5888;
pub const SIXEL_GDK_ERROR: ::std::os::raw::c_uint = 6144;
pub const SIXEL_GD_ERROR: ::std::os::raw::c_uint = 6400;
pub const SIXEL_LIBC_ERROR: ::std::os::raw::c_uint = 5120;
pub const SIXEL_CURL_ERROR: ::std::os::raw::c_uint = 5376;

pub type Status<T> = Result<T, Error>;

// NOTE: Sometimes a message can accompany errors.
// These can be acquired with `sixel_helper_get_additional_message`
#[derive(Debug)]
pub enum Error {
    False,
    Interrupted,
    BadAllocation,
    BadArgument,
    BadInput,
    Runtime,
    Logic,
    NotImplemented,
    Feature,
    STBI,
    STBIW,
    JPEG,
    PNG,
    GDK,
    GD,
    LibC,
    Curl,
    Other,
}

#[doc(hidden)]
pub fn from_libsixel(status: sixel_sys::status::Status) -> Status<()> {
    match status as u32 {
        SIXEL_OK => Ok(()),
        SIXEL_FALSE => Err(Error::False),
        SIXEL_INTERRUPTED => Err(Error::Interrupted),
        SIXEL_BAD_ALLOCATION => Err(Error::BadAllocation),
        SIXEL_BAD_ARGUMENT => Err(Error::BadArgument),
        SIXEL_BAD_INPUT => Err(Error::BadInput),
        SIXEL_RUNTIME_ERROR => Err(Error::Runtime),
        SIXEL_LOGIC_ERROR => Err(Error::Logic),
        SIXEL_NOT_IMPLEMENTED => Err(Error::NotImplemented),
        SIXEL_FEATURE_ERROR => Err(Error::Feature),
        SIXEL_STBI_ERROR => Err(Error::STBI),
        SIXEL_STBIW_ERROR => Err(Error::STBIW),
        SIXEL_JPEG_ERROR => Err(Error::JPEG),
        SIXEL_PNG_ERROR => Err(Error::PNG),
        SIXEL_GDK_ERROR => Err(Error::GDK),
        SIXEL_GD_ERROR => Err(Error::GD),
        SIXEL_LIBC_ERROR => Err(Error::LibC),
        SIXEL_CURL_ERROR => Err(Error::Curl),
        _ => panic!("Unkown sixel error"),
    }
}
