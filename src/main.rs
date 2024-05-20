use hex::decode;
fn main() {
    let hex_ciphertext = "160111";
    let ciphertext_bytes = decode(hex_ciphertext).expect("Invalid hex string");
    println!("{:?}", ciphertext_bytes);

    let plaintext = "The";
    let plaintext_bytes = plaintext.as_bytes();
    println!("{:?}", plaintext_bytes);

    let key_bytes: Vec<u8> = ciphertext_bytes
        .iter()
        .zip(plaintext_bytes.iter())
        .map(|(&c, &p)| c ^ p)
        .collect();

    let key_hex: Vec<String> = key_bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    println!("Key bytes in hex: {}", key_hex.join(""));
}
