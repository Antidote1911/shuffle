use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use shuffle_core::{PasswordConfig, generate_password, getversion};

#[no_mangle]
pub extern "C" fn get_version() -> *mut c_char {
    rust_to_c_string(getversion().to_string())
}

#[no_mangle]
pub extern "C" fn get_random(length: u8,uppercase: bool,lowercase:bool,digits:bool,symbols:bool,avoid: *mut c_char,) -> *mut c_char {
    let config = PasswordConfig::new(length as usize).unwrap()
        .with_uppercase(uppercase)
        .with_lowercase(lowercase)
        .with_digits(digits)
        .with_symbols(symbols)
        .avoid_ambiguous(c_to_rust_string(avoid).unwrap());

    let password = generate_password(&config);
    rust_to_c_string(password)
}


fn rust_to_c_string(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

fn c_to_rust_string(ptr: *mut c_char) -> Result<String, String> {
    let c_str: &CStr = unsafe { CStr::from_ptr(ptr) };
    let res = c_str
        .to_str()
        .map_err(|_| "Could not convert C string to Rust string".to_string())?
        .to_string();
    Ok(res)
}
