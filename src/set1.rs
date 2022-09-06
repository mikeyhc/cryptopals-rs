use cryptopals::bytestring::Bytestring;
use cryptopals::base64::Base64;
use cryptopals::cryptostring::Xor;
use cryptopals::hex::Hex;

pub fn run_all() {
    run_challenge_1();
    run_challenge_2();
}

pub fn run_challenge_1() {
    let input0 = "49276d206b696c6c696e6720796f757220627261696e206c6\
                  96b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected0 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlI\
                     GEgcG9pc29ub3VzIG11c2hyb29t";
    let bin0: Bytestring = Hex::try_from(input0).unwrap().into();
    println!("s1c1: {}", bin0.to_string());
    let output0 = Base64::from(&bin0).to_string();
    assert_eq!(expected0, output0)
}

pub fn run_challenge_2() {
    let input0 = Hex::try_from("1c0111001f010100061a024b53535009181c").unwrap();
    let input1 = Hex::try_from("686974207468652062756c6c277320657965").unwrap();
    let expected0 = "746865206b696420646f6e277420706c6179";

    let output0 = input0.xor(&input1);
    assert_eq!(expected0, output0.to_string());
    let bin0: Bytestring = output0.into();
    println!("s1c2: {}", bin0.to_string());
}
