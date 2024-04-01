use crate::dll::DLL;
use core::mem::transmute;
use cstr_core::{cstr, CStr};

#[derive(Clone, Copy)]
pub struct Libimg {}

impl Libimg {
    pub fn import(path: Option<&CStr>) -> Result<Self, &str> {
        let lib = DLL::load_dll(path.unwrap_or(cstr!("/sys/lib/libimg.obj")));
        match lib {
            Err(e) => return Err(e),
            Ok(dll) => unsafe { Ok(Libimg {}) },
        }
    }
}
