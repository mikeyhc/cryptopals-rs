use std::string::ToString;
use super::bytestring::Bytestring;

#[derive(Debug)]
pub struct Base64(String);

impl From<&Bytestring> for Base64 {
    fn from(b: &Bytestring) -> Self {
        let s = b.as_ref()
            .chunks(3)
            .map(Base64::bin_to_b64)
            .collect();
        Self(s)
    }
}

impl Base64 {
    fn bin_to_b64(b: &[u8]) -> String {
        match b.len() {
            // TODO simplify this with a common method
            3 => {
                let w = b[0] >> 2;
                let x = ((b[0] & 0x03) << 4) | ((b[1] & 0xF0) >> 4);
                let y = ((b[1] & 0x0F) << 2) | ((b[2] & 0xC0) >> 6);
                let z = b[2] & 0x3F;
                vec![
                    Base64::bin_to_char(w),
                    Base64::bin_to_char(x),
                    Base64::bin_to_char(y),
                    Base64::bin_to_char(z),
                ].iter().collect()
            },
            2 => {
                let w = b[0] >> 2;
                let x = ((b[0] & 0x03) << 4) | ((b[1] & 0xF0) >> 4);
                let y = (b[1] & 0x0F) << 2;
                vec![
                    Base64::bin_to_char(w),
                    Base64::bin_to_char(x),
                    Base64::bin_to_char(y),
                    '=',
                ].iter().collect()
            },
            1 => {
                let w = b[0] >> 2;
                let x = (b[0] & 0x03) << 4;
                vec![
                    Base64::bin_to_char(w),
                    Base64::bin_to_char(x),
                    '=',
                    '=',
                ].iter().collect()
            },
            _ => panic!() // impossible
        }
    }

    fn bin_to_char(c: u8) -> char {
        if c <= 25 {
            (c + 'A' as u8) as char
        } else if c >= 26 && c <= 51 {
            (c - 26 + 'a' as u8) as char
        } else if c >= 52 && c <= 61 {
            (c - 52 + '0' as u8) as char
        } else if c == 62 {
            '+'
        } else if c == 63 {
            '/'
        } else {
            panic!("c: {}", c) // impossible
        }
    }
}

impl ToString for Base64 {
    fn to_string(&self) -> String {
        let Base64(s) = self;
        s.to_string()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_from() {
        let input0 = super::Bytestring::from("Man");
        let super::Base64(s1) = super::Base64::from(&input0);
        assert_eq!(s1, "TWFu");
        let input1 = super::Bytestring::from("Ma");
        let super::Base64(s2) = super::Base64::from(&input1);
        assert_eq!(s2, "TWE=");
        let input2 = super::Bytestring::from("M");
        let super::Base64(s3) = super::Base64::from(&input2);
        assert_eq!(s3, "TQ==");
    }

    #[test]
    fn test_from_longform() {
        let input0 = super::Bytestring::from("Many hands make light work.");
        let output0 = "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu";
        let super::Base64(s0) = super::Base64::from(&input0);
        assert_eq!(output0, s0);

        let input1 = super::Bytestring::from("light work.");
        let output1 = "bGlnaHQgd29yay4=";
        let super::Base64(s1) = super::Base64::from(&input1);
        assert_eq!(output1, s1);

        let input2 = super::Bytestring::from("light work");
        let output2 = "bGlnaHQgd29yaw==";
        let super::Base64(s2) = super::Base64::from(&input2);
        assert_eq!(output2, s2);
    }
}
