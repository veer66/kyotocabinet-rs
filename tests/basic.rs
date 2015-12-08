extern crate kyotocabinet;

#[cfg(test)]
mod tests {
    use kyotocabinet as kc;
    #[test]
    fn test_basic() {
        let mut db = kc::DB::open("/tmp/titi.kch",
                              kc::Mode::WRITER as u32 |
                              kc::Mode::CREATE as u32)
            .ok().expect("Cannot open DB");
        db.set("key", "value");
        let value = db.get("key");
        assert!(&value.unwrap()[..] == "value");
    }
}
