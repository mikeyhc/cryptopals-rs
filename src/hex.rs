use std::fmt;
use std::convert::{Into, TryFrom};
use std::error::Error;
use std::string::ToString;

use super::bytestring::Bytestring;

#[derive(Debug, Clone)]
pub struct InvalidHexError(String);

impl fmt::Display for InvalidHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let InvalidHexError(s) = self;
        write!(f, "invalid hex string: {}", s)
    }
}

impl Error for InvalidHexError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct Hex(String);

impl TryFrom<&str> for Hex {
    type Error = InvalidHexError;

    fn try_from(s: &str) -> Result<Self, InvalidHexError> {
        // TODO: verify valid hex string
        if s.bytes().any(Hex::non_hexchar) {
            Err(InvalidHexError(s.to_string()))
        } else {
            Ok(Hex(s.to_string()))
        }
    }
}

impl Into<Bytestring> for Hex {
    fn into(self) -> Bytestring {
        let Hex(v) = self;
        let s: String = v.as_bytes()
            .chunks(2)
            .map(Hex::hexpair_to_bin)
            .map(|c| c as char)
            .collect();
        Bytestring::from(s.as_str())
    }
}

impl Hex {
    fn non_hexchar(c: u8) -> bool {
        !((c >= 'a' as u8 && c <= 'f' as u8) ||
            (c >= 'A' as u8 && c <= 'F' as u8) ||
            (c >= '0' as u8 && c <= '9' as u8))
    }

    fn hexpair_to_bin(hexpair: &[u8]) -> u8 {
        let a = Hex::hexchar_to_bin(hexpair[0]);
        let b = Hex::hexchar_to_bin(hexpair[1]);
        a << 4 | b
    }

    fn hexchar_to_bin(hexchar: u8) -> u8 {
        if hexchar >= 'a' as u8 && hexchar <= 'z' as u8 {
            hexchar - 'a' as u8 + 10
        } else if hexchar >= 'A' as u8 && hexchar <= 'Z' as u8 {
            hexchar - 'A' as u8 + 10
        } else if hexchar >= '0' as u8 && hexchar <= '9' as u8 {
            hexchar - '0' as u8
        } else {
            panic!("invalid hex state")
        }
    }
}

impl ToString for Hex {
    fn to_string(&self) -> String {
        let Hex(s) = self;
        s.to_string()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_from_str() {
        let h = super::Hex::try_from("000102").unwrap();
        let super::Hex(v) = h;
        assert_eq!("000102".to_string(), v);
    }

    #[test]
    fn test_conversion() {
        let h = super::Hex::try_from("0001020A0F").unwrap();
        let b: super::Bytestring = h.into();
        assert_eq!(vec![0, 1, 2, 10, 15], b.as_ref());
    }

    #[test]
    fn invalid_hex() {
        let s = "0001020A0F0X";
        let h = super::Hex::try_from(s);
        assert!(h.is_err());
        let e = match h {
            Err(super::InvalidHexError(e)) => e,
            Ok(_) => panic!()
        };
        assert_eq!(s, e);
    }
}
