extern crate libc;
use libc::c_char;
use std::collections::HashMap;
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
        let env_value = redhook::real!(getenv)(name);
        if env_value.is_null() {
            return env_value;
        }
        let name_str = CStr::from_ptr(name).to_str().unwrap();
        let name_string = name_str.to_string();
        // TODO: Replace this with a shared data structure from binary.
        let crypteia_envs: HashMap<String, String> = HashMap::from([
            ("SECRET".to_string(), "1A2B3C4D5E6F".to_string()),
        ]);
        if crypteia_envs.contains_key(&name_string) {
            let env_value_str = CStr::from_ptr(env_value).to_str().unwrap();
            let env_value_string = env_value_str.to_string();
            if env_value_string.starts_with("x-crypteia") {
                let crypteia_value = crypteia_envs.get(&name_string).unwrap().clone();
                let crypteia_value_c_string = CString::new(crypteia_value).unwrap();
                crypteia_value_c_string.into_raw()
            } else {
                env_value
            }
        } else {
            env_value
        }
    }
}
