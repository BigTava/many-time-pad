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
            str::from_utf8(&plaintext_bytes)
                .unwrap_or("[invalid UTF-8]")
                .to_string()
        })
        .collect()
}

fn main() {
    // Example hex-encoded ciphertexts
    let ciphertexts_hex = vec!["160111433b00035f", "050602061d07035f", "000000000000001a"];

    // Convert hex-encoded ciphertexts to bytes
    let ciphertexts: Vec<Vec<u8>> = ciphertexts_hex
        .iter()
        .map(|hex| hex_to_bytes(hex))
        .collect();

    // XOR the first two ciphertexts
    let xor_result = xor_bytes(&ciphertexts[0], &ciphertexts[1]);
    let xor_result2 = xor_bytes(&ciphertexts[0], &ciphertexts[2]);

    // Print XOR result
    println!("XOR result: {}", bytes_to_hex(&xor_result));
    println!("XOR result: {}", bytes_to_hex(&xor_result2));

    // Assuming some known plaintext to find the key
    let known_plaintext = b"aaaaaa"; // Example known plaintext
    let key = xor_bytes(&ciphertexts[0], known_plaintext);

    // Decrypt all messages using the found key
    let decrypted_messages = decrypt_with_key(&ciphertexts, &key);

    // Print decrypted messages
    for (i, message) in decrypted_messages.iter().enumerate() {
        println!("Decrypted message {}: {}", i + 1, message);
    }
}
