use cryptopals::bytestring::Bytestring;
use cryptopals::base64::Base64;
use cryptopals::hex::Hex;

pub fn run_all() {
    run_challenge_1();
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
