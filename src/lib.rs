extern crate libc;
use libc::*;
use std::ffi::{CString, CStr};
use std::str;
use std::fmt::Display;
use std::fmt;
use std::slice;

#[repr(C)]
pub struct KCDB {
    pub db: *mut c_void
}

pub enum Mode {
    READER = 1 << 0,
    WRITER = 1 << 1,
    CREATE = 1 << 2,
    TRUNCATE = 1 << 3,
    AUTOTRAN = 1 << 4,
    AUTOSYNC = 1 << 5,
    NOLOCK = 1 << 6,
    TRYLOCK = 1 << 7,
    NOREPAIR = 1 << 8
}

#[link(name = "kyotocabinet")]
extern {
    pub fn kcdbnew() -> *mut KCDB;
    pub fn kcdbdel(db: *mut KCDB);
    pub fn kcdbopen(db: *mut KCDB, path: *const c_char, mode: uint32_t) -> int32_t;
    pub fn kcdbclose(db: *mut KCDB) -> int32_t;
    pub fn kcdbemsg(db: *mut KCDB) -> *const c_char;
    pub fn kcdbset(db: *mut KCDB, kbuf: *const c_char, ksiz: size_t ,
                   vbuf: *const c_char, vsiz: size_t) -> int32_t;
    pub fn kcdbget(db: *mut KCDB, kbuf: *const c_char,
                   ksiz: size_t, sp: *mut size_t) -> *mut c_char;
}

#[derive(Debug, Clone)]
pub enum KCErrorType {
    OPEN
}

pub struct KCError {
    pub kind: KCErrorType,
    pub msg: String        
}

impl Display for KCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = self.kind.clone() as u32;
        f.write_fmt(format_args!("Error Type = {}, Error message: {}",
                                 kind, self.msg))
    }
}

pub struct DB {
    kcdb: *mut KCDB
}

impl DB {
    pub fn open(path: &str, mode: u32) -> Result<DB, KCError> {
        return unsafe {
            let db = kcdbnew();
            let _path = CString::new(path).unwrap();
            let ret = kcdbopen(db,
                               _path.as_ptr(),
                               mode);
            if ret == 0 {
                let msg = str::from_utf8(CStr::from_ptr(kcdbemsg(db))
                                         .to_bytes())
                    .unwrap()
                    .to_string();
                Err(KCError{kind: KCErrorType::OPEN,
                            msg: msg})
            } else {
                Ok(DB {kcdb: db})
            }
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> bool {
        unsafe {
            let _key = CString::new(key).unwrap();
            let _value = CString::new(value).unwrap();
            kcdbset(self.kcdb,
                    _key.as_ptr(), key.len(),
                    _value.as_ptr(), value.len()) != 0
        }
    }

    pub fn get_vec(&mut self, key: &str) -> Option<Vec<u8>> {
        let mut sp: usize = 0;
        let _key = CString::new(key).unwrap();
        let c_val = unsafe { kcdbget(self.kcdb, _key.as_ptr(), key.len(), &mut sp) };
        if c_val.is_null() {
            None
        } else {
            let u8_val = c_val as *const u8;
            let _val = unsafe {
                slice::from_raw_parts(u8_val, sp).to_vec()
            };
            Some(_val)
        }
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        match self.get_vec(key) {
            None => None,
            Some(vec_u8) => {
                unsafe { Some(String::from_utf8_unchecked(vec_u8)) }
            }
        }
    }
}

impl Drop for DB {
    fn drop(&mut self) {
        unsafe {
            kcdbclose(self.kcdb);
            kcdbdel(self.kcdb);            
        }
    }
}
