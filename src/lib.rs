extern crate libc;
mod log;
use lazy_static::lazy_static;
use libc::c_char;
use std::collections::HashMap;
use std::env::vars_os;
use std::ffi::{CStr, CString};
use std::fs::{metadata, remove_file, File};
use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, Once};

static mut DEBUG: bool = false;
static INITIALIZATION: Once = Once::new();

const ENV_FILE: &str = "/tmp/crypteia.json";
static ENV_FILE_READ: AtomicBool = AtomicBool::new(false);
lazy_static! {
    static ref CRYPTEIA_ENVS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

redhook::hook! {
    unsafe fn getenv(name: *const c_char) -> *mut c_char => crypteia_getenv {
        INITIALIZATION.call_once(|| {
            for (key, _value) in vars_os() {
                if let Some("CRYPTEIA_DEBUG") = key.to_str() {
                    DEBUG = true;
                }
            }
            if DEBUG {
                log::cloudwatch_metric("lib", "initialized", false, None);
            }
        });
        let env_value = redhook::real!(getenv)(name);
        if env_value.is_null() {
            return env_value;
        }
        let env_value_str = CStr::from_ptr(env_value).to_str().unwrap();
        let env_value_string = env_value_str.to_string();
        if env_value_string.starts_with("x-crypteia") {
            let name_str = CStr::from_ptr(name).to_str().unwrap();
            let name_string = name_str.to_string();
            if is_env_file_read() {
                match crypteia_env_value(name_string) {
                    Some(value) => value,
                    None => env_value
                }
            } else if is_env_file() {
                read_env_file();
                match crypteia_env_value(name_string) {
                  Some(value) => value,
                  None => env_value
                }
            } else {
                env_value
            }
        } else {
            env_value
        }
    }
}

fn crypteia_env_value(env: String) -> Option<*mut c_char> {
    if CRYPTEIA_ENVS.lock().unwrap().contains_key(&env) {
        let crypteia_value = CRYPTEIA_ENVS.lock().unwrap().get(&env).unwrap().clone();
        let crypteia_value_c_string = CString::new(crypteia_value).unwrap();
        Some(crypteia_value_c_string.into_raw())
    } else {
        None
    }
}

fn is_env_file() -> bool {
    let boo_value = metadata(ENV_FILE).is_ok();
    unsafe {
        if DEBUG {
            log::cloudwatch_metric("lib", "is_env_file", false, None);
        }
    }
    boo_value
}

fn is_env_file_read() -> bool {
    ENV_FILE_READ.load(Ordering::Relaxed)
}

fn read_env_file() {
    let mut file = File::open(ENV_FILE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let crypteia_envs: HashMap<String, String> = serde_json::from_str(&contents).unwrap();
    crypteia_envs.iter().for_each(|(env, value)| {
        CRYPTEIA_ENVS
            .lock()
            .unwrap()
            .insert(env.clone(), value.clone());
    });
    unsafe {
        if DEBUG {
            log::cloudwatch_metric("lib", "read_env_file", false, None);
        }
    }
    delete_file();
    ENV_FILE_READ.store(true, Ordering::Relaxed);
}

fn delete_file() {
    remove_file(ENV_FILE).unwrap();
    unsafe {
        if DEBUG {
            log::cloudwatch_metric("lib", "delete_file", false, None);
        }
    }
}
