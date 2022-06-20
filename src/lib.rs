extern crate libc;
use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::sync::Once;

static mut NOT_DEBUG: bool = true;
static INITIALIZATION: Once = Once::new();

redhook::hook! {
    unsafe fn getenv(name: *const c_char) -> *mut c_char => crypteia_getenv {
        INITIALIZATION.call_once(|| {
            for (key, _value) in std::env::vars_os() {
                if let Some("DEBUG") = key.to_str() {
                    NOT_DEBUG = false;
                }
            }
            if NOT_DEBUG {
                println!("[crypteia] Initialized libcrypteia using LD_PRELOAD");
            }
        });
        let original_value = redhook::real!(getenv)(name);
        if original_value.is_null() {
            return original_value;
        }
        let given_name = CStr::from_ptr(name).to_str().unwrap();
        if given_name == "HELLO" {
            // https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.as_ptr
            let result = CString::new("WORLD").unwrap();
            return result.into_raw();
        }
        original_value
    }
}
