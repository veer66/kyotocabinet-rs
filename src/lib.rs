extern crate libc;
use libc::*;

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
    pub fn kcdbset(db: *mut KCDB, kbuf: *const c_char, ksiz: size_t ,
                   vbuf: *const c_char, vsiz: size_t) -> int32_t;
    pub fn kcdbget(db: *mut KCDB, kbuf: *const c_char,
                   ksiz: size_t, sp: *mut size_t) -> *mut c_char;
}
