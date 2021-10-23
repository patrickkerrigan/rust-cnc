use std::os::raw::c_char;
use std::ffi::{CStr, CString};

mod dxf;
mod bmp;

const GCODE_HEADER: &'static str = "G01\n";
const GCODE_FOOTER: &'static str = "M05 F2000 X0 Y0";

#[no_mangle]
pub extern fn dxf_to_gcode(s: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let dxf = c_str.to_str().unwrap();

    let gcode = process(dxf);

    let c_str_gcode = CString::new(gcode).unwrap();
    c_str_gcode.into_raw()
}

#[no_mangle]
pub extern fn gcode_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return
        }

        CString::from_raw(s)
    };
}

pub fn process(dxf_contents: &str) -> String {
    wrap_gcode(dxf::dxf_to_gcode(dxf_contents).as_str())
}


pub fn process_bmp(bmp_contents: &[u8], dpi: u16) -> String {
    wrap_gcode(bmp::bmp_to_gcode(bmp_contents, dpi).as_str())
}

fn wrap_gcode(gcode: &str) -> String {
    String::from(GCODE_HEADER)
        + gcode
        + GCODE_FOOTER
}


