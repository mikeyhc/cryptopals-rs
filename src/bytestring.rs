use std::collections::HashMap;
use std::convert::{AsRef, From};
use std::iter::zip;
use std::string::ToString;

use super::cryptostring::{Score, Xor};

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

impl Xor<u8> for Bytestring {
    fn xor(&self, other: &u8) -> Bytestring {
        let Bytestring(inner) = self;
        let s = vec![*other].iter()
            .cycle()
            .take(inner.len())
            .map(|x| *x)
            .collect();
        self.xor(&Bytestring(s))
    }
}

impl Score for Bytestring {
    fn score(&self) -> f32 {
        let Bytestring(s) = self;
        let counts = s.iter()
            .fold(HashMap::new(), |mut m, c| {
                match m.get(c)  {
                    None => m.insert(c, 0),
                    Some(v) => m.insert(c, v+1)
                };
                m
            });
        counts.iter()
            .map(|(k, v)| (k, *v as f32 / s.len() as f32))
            .map(|(k, v)| (v - char_score(**k as char)).abs())
            .sum()
    }
}

fn char_score(c: char) -> f32 {
    match c {
        'a' | 'A' => 8.2,
        'b' | 'B' => 1.5,
        'c' | 'C' => 2.8,
        'd' | 'D' => 4.3,
        'e' | 'E' => 13.0,
        'f' | 'F' => 2.2,
        'g' | 'G' => 2.0,
        'h' | 'H' => 6.1,
        'i' | 'I' => 7.0,
        'j' | 'J' => 0.15,
        'k' | 'K' => 0.77,
        'l' | 'L' => 4.0,
        'm' | 'M' => 2.5,
        'n' | 'N' => 6.7,
        'o' | 'O' => 7.5,
        'p' | 'P' => 1.9,
        'q' | 'Q' => 0.095,
        'r' | 'R' => 6.0,
        's' | 'S' => 6.3,
        't' | 'T' => 9.1,
        'u' | 'U' => 2.8,
        'v' | 'V' => 0.98,
        'w' | 'W' => 2.4,
        'x' | 'X' => 0.15,
        'y' | 'Y' => 2.0,
        'z' | 'Z' => 0.074,
        _         => 100.0
    }
}

#[cfg(test)]
mod test {
    use super::Bytestring;
    use super::{Score, Xor};

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

    #[test]
    fn test_score() {
        let input0 = Bytestring(vec![0, 0, 0]);
        let output0 = input0.score();
        assert!((100.0 - output0).abs() < 1.0);

        let input1 = Bytestring("abc".bytes().collect());
        let output1 = input1.score();
        assert_eq!(12.5, output1);
    }
}
