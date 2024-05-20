use std::str;

fn xorstr(word: &str, segment: &[u8]) -> Vec<u8> {
    word.as_bytes()
        .iter()
        .zip(segment)
        .map(|(a, b)| a ^ b)
        .collect()
}

fn cribdrag<F>(cipher: &str, word: &str, mut callback: F)
where
    F: FnMut(String),
{
    let cipher_bytes = hex::decode(cipher).expect("Invalid hex string");
    let word_len = word.len();

    for window in cipher_bytes.windows(word_len) {
        let xored = xorstr(word, window);
        if let Ok(xored_str) = str::from_utf8(&xored) {
            callback(xored_str.to_string());
        } else {
            callback(format!("{:?}", xored));
        }
    }
}

fn main() {
    let xorcipher = "050602061d07035f4e3553501400004c1e4f1f01451359540c5804110c1c47560a1415491b06454f0e45040816431b144f0f4900450d1501094c1b16550f0b4e151e03031b450b4e020c1a124f020a0a4d09071f16003a0e5011114501494e16551049021011114c291236520108541801174b03411e1d124554284e141a0a1804045241190d543c00075453020a044e134f540a174f1d080444084e01491a090b0a1b4103570740";
    let crib =
        "Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial";

    cribdrag(xorcipher, crib, |chunk| {
        println!("{}", chunk);
    });
}

// "Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution";
