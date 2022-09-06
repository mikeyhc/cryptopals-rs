use std::convert::{AsRef, From};
use std::iter::zip;
use std::string::ToString;

use super::cryptostring::Xor;

#[derive(Debug)]
pub struct Bytestring(Vec<u8>);

impl AsRef<[u8]> for Bytestring {
    fn as_ref(&self) -> &[u8] {
        let Bytestring(s) = self;
        &s
    }
}

impl From<&Vec<u8>> for Bytestring {
    fn from(b: &Vec<u8>) -> Self {
        Bytestring(b.clone())
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

impl Xor<Bytestring> for Bytestring {
    fn xor(&self, other: &Bytestring) -> Bytestring {
        Bytestring(
            zip(self.as_ref(), other.as_ref())
            .map(|(x, y)| x ^ y)
            .collect())
    }
}

#[cfg(test)]
mod test {
    use super::Bytestring;
    use super::Xor;

    #[test]
    fn test_xor() {
        let input0 = Bytestring(vec![0, 0, 0, 0]);
        let input1 = Bytestring(vec![1, 1, 1, 1]);
        let expected0 = vec![1, 1, 1, 1];
        let Bytestring(output1) = input0.xor(&input1);
        assert_eq!(expected0, output1);

        let input2 = Bytestring(vec![3, 4, 5, 6]);
        let input3 = Bytestring(vec![6, 5, 4, 3]);
        let expected1 = vec![5, 1, 1, 5];
        let Bytestring(output1) = input2.xor(&input3);
        assert_eq!(expected1, output1);

        let input4 = Bytestring(vec![3, 4, 5, 6]);
        let input5 = Bytestring(vec![3, 4, 5, 6]);
        let expected2 = vec![0, 0, 0, 0];
        let Bytestring(output2) = input4.xor(&input5);
        assert_eq!(expected2, output2);

        let empty_vec: Vec<u8> = vec![];
        let input6 = Bytestring(empty_vec.clone());
        let Bytestring(output3) = input6.xor(&input6);
        assert_eq!(empty_vec, output3)
    }
}
