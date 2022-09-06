use std::convert::{AsRef, From};
use std::string::ToString;

#[derive(Debug)]
pub struct Bytestring(Vec<u8>);

impl AsRef<[u8]> for Bytestring {
    fn as_ref(&self) -> &[u8] {
        let Bytestring(s) = self;
        &s
    }
}

impl From<&str> for Bytestring {
    fn from(b: &str) -> Self {
        Bytestring(b.as_bytes().to_vec())
    }
}

impl ToString for Bytestring {
    fn to_string(&self) -> String {
        let Bytestring(s) = self;
        s.iter().map(|c| *c as char).collect()
    }
}
