extern crate kyotocabinet;

#[cfg(test)]
mod tests {
    use kyotocabinet as kc;
    use std::ffi::CString;
    use std::ffi::CStr;
    use std::str;
    
    #[test]
    fn test_new_del_open_db() {
        let path = CString::new("/tmp/foo.kch")
            .expect("Cannot new CString");
         unsafe {
             let db = kc::kcdbnew();
             let ret = kc::kcdbopen(db,
                                    path.as_ptr(),
                                    kc::Mode::WRITER as u32 |
                                    kc::Mode::CREATE as u32);
             assert!(ret != 0);
             kc::kcdbclose(db);
             kc::kcdbdel(db);
         }
    }
    
    #[test]
    fn test_set() {
        let path = CString::new("/tmp/foo.kch")
            .expect("Cannot new CString");
        let key = CString::new("key")
            .expect("Cannot new key");
        let val = CString::new("value")
            .expect("Cannot new value");
        let mut sp: usize = 0;
        unsafe {
            let db = kc::kcdbnew();
            let ret = kc::kcdbopen(db,
                                   path.as_ptr(),
                                   kc::Mode::WRITER as u32 |
                                   kc::Mode::CREATE as u32);
            assert!(ret != 0);
            kc::kcdbset(db, key.as_ptr(), 3, val.as_ptr(), 5);
            let c_val = kc::kcdbget(db, key.as_ptr(), 3, &mut sp);
            assert!(sp == 5);
            assert!(!c_val.is_null());
            let _val = str::from_utf8(CStr::from_ptr(c_val)
                                      .to_bytes())
                .unwrap();
            assert!(_val == "value");
            kc::kcdbclose(db);
            kc::kcdbdel(db);
         }
    }
}
