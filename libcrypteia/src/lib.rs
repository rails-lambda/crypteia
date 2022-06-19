extern crate libc;

use std::ffi::CStr;
use std::ffi::CString;

redhook::hook! {
    unsafe fn getenv(name: *const libc::c_char) -> *mut libc::c_char => dangerously_mutate_getenv {
      println!("Hello libcrypteia!");
      let given_pointer = redhook::real!(getenv)(name);
      if given_pointer.is_null() {
          return given_pointer;
      }
      let given_name = CStr::from_ptr(name).to_str().unwrap();
      // let given_value = CStr::from_ptr(given_pointer).to_str().unwrap();
      // println!("Got: {}:{}", given_name, given_value);
      if given_name == "HELLO" {
          // https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.as_ptr
          let result = CString::new("WORLD").unwrap();
          return result.into_raw();
      }
      return given_pointer;
    }
}
