use std::fmt::Write;
use std::str;

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn xor_bytes(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect()
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex_string = String::new();
    for byte in bytes {
        write!(&mut hex_string, "{:02x}", byte).unwrap();
    }
    hex_string
}

fn decrypt_with_key(ciphertexts: &[Vec<u8>], key: &[u8]) -> Vec<String> {
    ciphertexts
        .iter()
        .map(|ct| {
            let plaintext_bytes: Vec<u8> = xor_bytes(ct, key);
            str::from_utf8(&plaintext_bytes).unwrap_or("[invalid UTF-8]").to_string()
        })
        .collect()
}

fn main() {
    // Example hex-encoded ciphertexts
    let line_1 =
        "160111433b00035f536110435a380402561240555c526e1c0e431300091e4f04451d1d490d1c49010d000a0a4510111100000d434202081f0755034f13031600030d0204040e";
    let line_2 =
        "050602061d07035f4e3553501400004c1e4f1f01451359540c5804110c1c47560a1415491b06454f0e45040816431b144f0f4900450d1501094c1b16550f0b4e151e03031b450b4e020c1a124f020a0a4d09071f16003a0e5011114501494e16551049021011114c291236520108541801174b03411e1d124554284e141a0a1804045241190d543c00075453020a044e134f540a174f1d080444084e01491a090b0a1b4103570740";
    let line_3 =
        "000000000000001a49320017071704185941034504524b1b1d40500a0352441f021b0708034e4d0008451c40450101064f071d1000100201015003061b0b444c00020b1a16470a4e051a4e114f1f410e08040554154f064f410c1c00180c0010000b0f5216060605165515520e09560e00064514411304094c1d0c411507001a1b45064f570b11480d001d4c134f060047541b185c";
    let line_4 =
        "0b07540c1d0d0b4800354f501d131309594150010011481a1b5f11090c0845124516121d0e0c411c030c45150a16541c0a0b0d43540c411b0956124f0609075513051816590026004c061c014502410d024506150545541c450110521a111758001d0607450d11091d00121d4f0541190b45491e02171a0d49020a534f";
    let line_5 =
        "031a5410000a075f5438001210110a011c5350080a0048540e431445081d521345111c041f0245174a0006040002001b01094914490f0d53014e570214021d00160d151c57420a0d03040b4550020e1e1f001d071a56110359420041000c0b06000507164506151f104514521b02000b0145411e05521c1852100a52411a0054180a1e49140c54071d5511560201491b0944111a011b14090c0e41";
    let line_6 =
        "0b4916060808001a542e0002101309050345500b00050d04005e030c071b4c1f111b161a4f01500a08490b0b451604520d0b1d1445060f531c48124f1305014c051f4c001100262d38490f0b4450061800004e001b451b1d594e45411d014e004801491b0b0602050d41041e0a4d53000d0c411c41111c184e130a0015014f03000c1148571d1c011c55034f12030d4e0b45150c5c";
    let line_7 =
        "011b0d131b060d4f5233451e161b001f59411c090a0548104f431f0b48115505111d17000e02000a1e430d0d0b04115e4f190017480c14074855040a071f4448001a050110001b014c1a07024e5014094d0a1c541052110e54074541100601014e101a5c";
    let line_8 = "0c06004316061b48002a4509065e45221654501c0a075f540c42190b165c";

    let ciphertexts_hex = vec![line_1, line_2, line_3, line_4, line_5, line_6, line_7, line_8];

    for i in 0..ciphertexts_hex.len() {
        for j in i + 1..ciphertexts_hex.len() {
            let bytes_i = hex_to_bytes(ciphertexts_hex[i]);
            let bytes_j = hex_to_bytes(ciphertexts_hex[j]);

            let xor_result = xor_bytes(&bytes_i, &bytes_j);
            println!("XOR result between line {i} and line {j}: {}", bytes_to_hex(&xor_result));
        }
    }

    let common_4_letter_words: Vec<&str> = vec![
        "with",
        "have",
        "this",
        "they",
        "will",
        "your",
        "from",
        "know",
        "what"
    ];

    let given_hex = "0e43";
    let bytes_given = hex_to_bytes(given_hex);

    for word in common_4_letter_words {
        println!("{}", word);
        let bytes_word = hex_to_bytes(&word);
        let xor_result = xor_bytes(&bytes_word, &bytes_given);
        println!("XOR result between {word} and {given_hex}: {}", bytes_to_hex(&xor_result));
    }
}
