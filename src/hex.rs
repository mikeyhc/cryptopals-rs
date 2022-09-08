use std::fmt;
use std::convert::{From, Into, TryFrom};
use std::error::Error;
use std::string::ToString;

use super::bytestring::Bytestring;
use super::cryptostring::{Score, Xor};

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

#[derive(Clone, Debug)]
pub struct Hex(String);

impl From<&Bytestring> for Hex {
    fn from(b: &Bytestring) -> Hex {
        let s = b.as_ref().iter()
            .flat_map(Hex::bin_to_hexpair)
            .collect();
        Hex(s)
    }
}

impl TryFrom<&str> for Hex {
    type Error = InvalidHexError;

    fn try_from(s: &str) -> Result<Self, InvalidHexError> {
        if s.bytes().any(Hex::non_hexchar) {
            Err(InvalidHexError(s.to_string()))
        } else {
            Ok(Hex(s.to_string()))
        }
    }
}

impl Into<Bytestring> for &Hex {
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

    fn bin_to_hexpair(binchar: &u8) -> Vec<char> {
        let b1 = binchar >> 4;
        let b2 = binchar & 0x0F;
        vec![b1, b2].iter()
            .map(|&c| {
                (if c > 9 { c - 10 + 'a' as u8} else { c + '0' as u8}) as char
            })
            .collect()
    }
}

impl ToString for Hex {
    fn to_string(&self) -> String {
        let Hex(s) = self;
        s.to_string()
    }
}

impl Xor<Hex> for Hex {
    fn xor(&self, other: &Hex) -> Hex {
        let b0: Bytestring = self.into();
        let b1: Bytestring = other.into();
        Hex::from(&b0.xor(&b1))
    }
}

impl Xor<u8> for Hex {
    fn xor(&self, other: &u8) -> Hex {
        let b0: Bytestring = self.into();
        Hex::from(&b0.xor(other))
    }
}

impl Score for Hex {
    fn score(&self) -> f32 {
        let b0: Bytestring = self.into();
        b0.score()
    }
}

#[cfg(test)]
mod test {
    use super::{Hex, InvalidHexError};
    use super::Bytestring;

    #[test]
    fn test_from_str() {
        let h = Hex::try_from("000102").unwrap();
        let Hex(v) = h;
        assert_eq!("000102".to_string(), v);
    }

    #[test]
    fn test_conversion() {
        let h = Hex::try_from("0001020A0F").unwrap();
        let b: Bytestring = h.into();
        assert_eq!(vec![0, 1, 2, 10, 15], b.as_ref());
    }

    #[test]
    fn invalid_hex() {
        let s = "0001020A0F0X";
        let h = Hex::try_from(s);
        assert!(h.is_err());
        let e = match h {
            Err(InvalidHexError(e)) => e,
            Ok(_) => panic!()
        };
        assert_eq!(s, e);
    }

    #[test]
    fn from_bytestring() {
        let u8vec: Vec<u8> = vec![0, 1, 2, 10, 255];
        let input0 = Bytestring::from(&u8vec);
        let expected0 = "0001020aff";
        let Hex(output0) = Hex::from(&input0);
        assert_eq!(expected0, output0)
    }
}
