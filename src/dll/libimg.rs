use crate::dll::DLL;
use core::mem::transmute;
use cstr_core::{cstr, CStr};

#[derive(Clone, Copy)]
pub struct Libimg {
    img_from_file: extern "stdcall" fn(*const u8),
}

impl Libimg {
    pub fn import(path: Option<&CStr>) -> Result<Self, &str> {
        let lib = DLL::load_dll(path.unwrap_or(cstr!("/sys/lib/libimg.obj")));
        match lib {
            Err(e) => return Err(e),
            Ok(dll) => unsafe {
                Ok(Libimg {
                    img_from_file: transmute(dll.get_func(cstr!("img_from_file")).ok().unwrap()),
                })
            },
        }
    }
    pub fn from_file(&self, filename: *const u8) {
        (self.img_from_file)(filename);
    }
}
